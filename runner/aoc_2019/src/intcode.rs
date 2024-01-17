use helper::{Error, Lines};
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub(crate) trait Word:
    Copy + Clone + FromStr + std::fmt::Display + std::fmt::Debug + PartialEq + PartialOrd
{
    const ZERO: Self;
    const ONE: Self;
    fn usize(self) -> usize;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

macro_rules! impl_word {
    ($ty:ty) => {
        impl Word for $ty {
            const ZERO: $ty = 0;
            const ONE: $ty = 1;

            fn usize(self) -> usize {
                self as usize
            }

            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            fn wrapping_mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
        }
    };

    ($ty:ty, $($tt:tt)+) => {
        impl_word!($ty);
        impl_word!($($tt)+);
    };
}

impl_word!(u8, u16, u32, u64, u128, usize);
impl_word!(i8, i16, i32, i64, i128, isize);

#[derive(Default, Copy, Clone, Debug)]
pub(crate) enum State<T> {
    #[default]
    Running,
    WaitingForInput(T, u8),
    HasOutput(T),
    Stopped,
}

#[derive(Default, Clone)]
pub(crate) struct IntCode<T> {
    pc: usize,
    state: State<T>,
    mem: Vec<T>,
    relbase: T,
    pub(crate) input: VecDeque<T>,
}

impl<T> Deref for IntCode<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

impl<T> DerefMut for IntCode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mem
    }
}

impl<T> IntCode<T>
where
    T: Word,
    helper::Error: From<<T as FromStr>::Err>,
{
    pub(crate) fn load(&mut self, lines: Lines) -> Result<(), Error> {
        self.mem.clear();
        for line in lines.iter() {
            for num in line.split(',') {
                if num.is_empty() {
                    continue;
                }
                self.mem.push(num.parse()?)
            }
        }

        Ok(())
    }

    pub(crate) fn run(&mut self) -> State<T> {
        loop {
            match self.state {
                State::Stopped => return self.state,
                State::WaitingForInput(rd, mode) => {
                    if let Some(v) = self.input.pop_front() {
                        self.write_mem(rd, v, mode);
                    } else {
                        return self.state;
                    }
                    self.state = State::Running;
                    self.tick();
                }
                State::HasOutput(v) => {
                    self.state = State::Running;
                    return State::HasOutput(v);
                }
                State::Running => self.tick(),
            }
        }
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.state, State::Stopped)
    }

    fn get_from_pc(&mut self) -> T {
        let val = self.mem[self.pc];
        self.pc += 1;
        val
    }

    fn read_mem(&self, addr: usize) -> T {
        if let Some(v) = self.mem.get(addr) {
            *v
        } else {
            T::ZERO
        }
    }

    fn write_mem(&mut self, addr: T, v: T, mode: u8) {
        let addr = match mode {
            0 => addr,
            1 => panic!("Cannot write to immediate"),
            2 => self.relbase.wrapping_add(addr),
            _ => unreachable!(),
        };
        if addr < T::ZERO {
            panic!("Address is less than 0.  {addr}");
        }
        let addr = addr.usize();
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, T::ZERO);
        }
        self.mem[addr] = v;
    }

    fn get_arg(&mut self, mode: u8) -> T {
        let arg = self.get_from_pc();
        match mode {
            0 => self.read_mem(arg.usize()),
            1 => arg,
            2 => {
                let mem_loc = self.relbase.wrapping_add(arg);
                if mem_loc < T::ZERO {
                    panic!("Trying to access memory below 0");
                }
                self.read_mem(mem_loc.usize())
            }
            _ => unreachable!(),
        }
    }

    fn tick(&mut self) {
        let opcode = self.get_from_pc().usize();
        let mode_p3 = ((opcode / 10000) % 10) as u8;
        let mode_p2 = ((opcode / 1000) % 10) as u8;
        let mode_p1 = ((opcode / 100) % 10) as u8;
        let opcode = opcode % 100;
        match opcode {
            1 => {
                // Add rs1 rs2 rd
                let rs1 = self.get_arg(mode_p1);
                let rs2 = self.get_arg(mode_p2);
                let rd = self.get_from_pc();
                self.write_mem(rd, rs1.wrapping_add(rs2), mode_p3);
            }
            2 => {
                // Mul rs1 rs2 rd
                let rs1 = self.get_arg(mode_p1);
                let rs2 = self.get_arg(mode_p2);
                let rd = self.get_from_pc();
                self.write_mem(rd, rs1.wrapping_mul(rs2), mode_p3);
            }
            3 => {
                // Input rd
                let rd = self.get_from_pc();
                if let Some(v) = self.input.pop_front() {
                    self.write_mem(rd, v, mode_p1);
                } else {
                    self.state = State::WaitingForInput(rd, mode_p1);
                }
            }
            4 => {
                // Output rs1
                let rs1 = self.get_arg(mode_p1);
                self.state = State::HasOutput(rs1);
            }
            5 => {
                // Jump-if-true rs1 target_pc
                let rs1 = self.get_arg(mode_p1);
                let target_pc = self.get_arg(mode_p2).usize();

                if rs1 != T::ZERO {
                    self.pc = target_pc;
                }
            }
            6 => {
                // Jump-if-false rs1 target_pc
                let rs1 = self.get_arg(mode_p1);
                let target_pc = self.get_arg(mode_p2).usize();

                if rs1 == T::ZERO {
                    self.pc = target_pc;
                }
            }
            7 => {
                // Less-than rs1 rs2 rd
                let rs1 = self.get_arg(mode_p1);
                let rs2 = self.get_arg(mode_p2);
                let rd = self.get_from_pc();

                self.write_mem(rd, if rs1 < rs2 { T::ONE } else { T::ZERO }, mode_p3);
            }
            8 => {
                // Equals rs1 rs2 rd
                let rs1 = self.get_arg(mode_p1);
                let rs2 = self.get_arg(mode_p2);
                let rd = self.get_from_pc();

                self.write_mem(rd, if rs1 == rs2 { T::ONE } else { T::ZERO }, mode_p3);
            }
            9 => {
                // Adjust relbase rs1
                let rs1 = self.get_arg(mode_p1);
                self.relbase = self.relbase.wrapping_add(rs1);
            }
            99 => self.state = State::Stopped,
            _ => unreachable!(),
        }
    }
}
