#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day21 {
    player: [(usize, usize); 2],
}

impl Day21 {
    pub fn new() -> Self {
        Self {
            player: [(0, 0); 2],
        }
    }
}

struct QuantumDice {
    rolls: Vec<(usize, usize)>,
}

impl QuantumDice {
    fn new() -> Self {
        let mut totals = Vec::new();
        let mut universes = Vec::new();
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let sum = a + b + c;

                    let mut found = false;
                    for i in 0..totals.len() {
                        if totals[i] == sum {
                            found = true;
                            universes[i] += 1;
                            break;
                        }
                    }

                    if !found {
                        totals.push(sum);
                        universes.push(1);
                    }
                }
            }
        }
        Self {
            rolls: totals
                .iter()
                .copied()
                .zip(universes.iter().copied())
                .collect(),
        }
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 2);
        fn parse(line: &str) -> Result<usize, Error> {
            Ok(line.split_whitespace().last().unwrap().parse()?)
        }
        self.player[0].0 = parse(lines[0].as_str())?;
        self.player[1].0 = parse(lines[1].as_str())?;
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day21 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        struct Dice(usize, usize);
        impl Dice {
            fn next(&mut self) -> usize {
                self.1 += 1;
                if self.0 == 100 {
                    self.0 = 1;
                } else {
                    self.0 += 1;
                }
                self.0
            }

            fn rolls(&self) -> usize {
                self.1
            }
        }
        let mut dice = Dice(0, 0);
        let mut i = 0;
        loop {
            let roll = dice.next() + dice.next() + dice.next();
            self.player[i].0 = (((self.player[i].0 - 1) + roll) % 10) + 1;
            self.player[i].1 += self.player[i].0;

            if self.player[i].1 >= 1000 {
                break;
            }
            i = 1 - i;
        }
        Ok((self.player[0].1.min(self.player[1].1) * dice.rolls()).into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let dice = QuantumDice::new();

        fn play(
            dice: &QuantumDice,
            players: [(usize, usize); 2],
            universes: usize,
            mut player: usize,
            wins: &mut [usize; 2],
        ) {
            player = 1 - player;

            for roll in dice.rolls.iter() {
                let mut pos = players[player].0 + roll.0;
                pos = ((pos - 1) % 10) + 1;
                let universes = universes * roll.1;
                let mut players = players;
                players[player].0 = pos;
                players[player].1 += pos;

                if players[player].1 >= 21 {
                    wins[player] += universes;
                } else {
                    play(dice, players, universes, player, wins);
                }
            }
        }

        let mut wins = [0; 2];
        play(&dice, self.player, 1, 1, &mut wins);

        Ok(wins[0].max(wins[1]).into())
    }
}
