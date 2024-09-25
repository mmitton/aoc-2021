#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::VecDeque;

pub struct Day22 {
    players: [VecDeque<u8>; 2],
}

impl Day22 {
    pub fn new() -> Self {
        Self {
            players: std::array::from_fn(|_| VecDeque::default()),
        }
    }

    fn score(&self, player: usize) -> usize {
        self.players[player]
            .iter()
            .rev()
            .copied()
            .enumerate()
            .fold(0, |score, (i, v)| score + ((i + 1) * v as usize))
    }
}

impl Runner for Day22 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::REMOVE_EMPTY)?;
        let mut player: usize = 0;

        for line in lines.iter() {
            if let Some(p) = line.strip_prefix("Player ") {
                player = p[..p.len() - 1].parse::<usize>()? - 1;
            } else {
                self.players[player].push_back(line.parse()?);
            }
        }
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        loop {
            match (self.players[0].pop_front(), self.players[1].pop_front()) {
                (Some(p0), Some(p1)) => {
                    if p0 > p1 {
                        self.players[0].extend([p0, p1].iter());
                    } else {
                        self.players[1].extend([p1, p0].iter());
                    }
                }
                (Some(p0), None) => {
                    self.players[0].push_front(p0);
                    return Ok(self.score(0).into());
                }
                (None, Some(p1)) => {
                    self.players[1].push_front(p1);
                    return Ok(self.score(1).into());
                }
                _ => unreachable!(),
            }
        }
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let mut results = HashMap::default();
        let winner = recursive_combat(&mut self.players, &mut results);

        Ok(self.score(winner).into())
    }
}

fn recursive_combat(
    players: &mut [VecDeque<u8>; 2],
    results: &mut HashMap<[VecDeque<u8>; 2], usize>,
) -> usize {
    let initial_players = players.clone();
    if let Some(winner) = results.get(players) {
        return *winner;
    }

    let mut seen = HashSet::default();

    if !results.is_empty() {
        let p1_max = *players[0].iter().max().unwrap() as usize;
        let p2_max = *players[1].iter().max().unwrap() as usize;
        if p1_max > p2_max && p1_max > players[0].len() + players[1].len() - 2 {
            results.insert(initial_players, 0);
            return 0;
        }
    }

    loop {
        if !seen.insert(players.clone()) {
            results.insert(initial_players, 0);
            return 0;
        }

        let p1 = players[0].pop_front().unwrap() as usize;
        let p2 = players[1].pop_front().unwrap() as usize;

        let winner = if p1 <= players[0].len() && p2 <= players[1].len() {
            let mut sub_players: [VecDeque<u8>; 2] =
                std::array::from_fn(|i| VecDeque::with_capacity(if i == 0 { p1 } else { p2 }));
            sub_players[0].extend(players[0].iter().take(p1));
            sub_players[1].extend(players[1].iter().take(p2));
            assert_eq!(sub_players[0].len(), sub_players[0].capacity());
            assert_eq!(sub_players[1].len(), sub_players[1].capacity());

            recursive_combat(&mut sub_players, results)
        } else if p1 > p2 {
            0
        } else {
            1
        };

        if winner == 0 {
            players[0].push_back(p1 as u8);
            players[0].push_back(p2 as u8);
        } else {
            players[1].push_back(p2 as u8);
            players[1].push_back(p1 as u8);
        }

        if players[0].is_empty() || players[1].is_empty() {
            break;
        }
    }

    let winner = if players[0].is_empty() { 1 } else { 0 };
    results.insert(initial_players, winner);
    winner
}
