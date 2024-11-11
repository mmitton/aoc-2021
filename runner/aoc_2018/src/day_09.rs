#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day09 {
    players: usize,
    last_marble_points: usize,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn play(&self) -> (usize, usize) {
        let mut scores = vec![0; self.players];

        struct Node {
            v: usize,
            n: usize,
            p: usize,
        }

        let mut linked_list = vec![Node { v: 0, n: 0, p: 0 }];
        let mut cur = 0;
        let mut head = 0;
        let mut player = 0;

        macro_rules! move_cur {
            ($offset:expr) => {
                if $offset < 0 {
                    for _ in $offset..0 {
                        cur = linked_list[cur].p;
                    }
                } else {
                    for _ in 0..$offset {
                        cur = linked_list[cur].n;
                    }
                }
            };
        }

        for i in 1..=self.last_marble_points {
            if i % 23 != 0 {
                cur = linked_list[cur].n;
                let next = linked_list[cur].n;
                let prev = cur;

                let new_node = Node {
                    v: i,
                    n: next,
                    p: prev,
                };

                linked_list.push(new_node);
                cur = linked_list.len() - 1;
                linked_list[prev].n = cur;
                linked_list[next].p = cur;
            } else {
                for _ in -7..0 {
                    cur = linked_list[cur].p;
                }
                scores[player] += i + linked_list[cur].v;

                let next = linked_list[cur].n;
                let prev = linked_list[cur].p;
                linked_list[next].p = prev;
                linked_list[prev].n = next;
                if head == cur {
                    head = next;
                }
                cur = next;
            }

            player = (player + 1) % self.players;
        }

        scores
            .iter()
            .copied()
            .enumerate()
            .fold((0, 0), |high, (player, score)| {
                if score > high.1 {
                    (player, score)
                } else {
                    high
                }
            })
    }
}

impl Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);

        let parts: Vec<&str> = lines[0].split(" ").collect();
        self.players = parts[0].parse()?;
        self.last_marble_points = parts[6].parse()?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self.play().1.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        self.last_marble_points *= 100;
        Ok(self.play().1.into())
    }
}
