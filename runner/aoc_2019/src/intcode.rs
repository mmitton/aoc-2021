use helper::{Error, Lines};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub trait Word: Copy + Clone + FromStr {
    fn usize(self) -> usize;
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

macro_rules! impl_word {
    ($ty:ty) => {
        impl Word for $ty {
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

#[derive(Default, Clone)]
pub struct IntCode<T> {
    pc: usize,
    stop: bool,
    mem: Vec<T>,
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
    pub fn load(&mut self, lines: Lines) -> Result<(), Error> {
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

    pub fn run(&mut self) {
        while !self.stop {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.mem[self.pc].usize();
        self.pc += 1;
        match opcode {
            1 => {
                // Add reg reg reg
                let rs1 = self.mem[self.pc].usize();
                let rs2 = self.mem[self.pc + 1].usize();
                let rd = self.mem[self.pc + 2].usize();
                self.mem[rd] = self.mem[rs1].wrapping_add(self.mem[rs2]);
                self.pc += 3;
            }
            2 => {
                // Mul reg reg reg
                let rs1 = self.mem[self.pc].usize();
                let rs2 = self.mem[self.pc + 1].usize();
                let rd = self.mem[self.pc + 2].usize();
                self.mem[rd] = self.mem[rs1].wrapping_mul(self.mem[rs2]);
                self.pc += 3;
            }
            99 => self.stop = true,
            _ => unreachable!(),
        }
    }
}
