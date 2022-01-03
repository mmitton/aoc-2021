use std::collections::VecDeque;

fn main() {
    let num_elves = if cfg!(debug_assertions) { 5 } else { 3017957 };

    let mut elves = VecDeque::new();
    let mut next_elves = VecDeque::new();
    for elf in 1..=num_elves {
        elves.push_back((elf, 1));
    }
    loop {
        next_elves.clear();
        for _ in (0..elves.len()).step_by(2) {
            let cur = elves.pop_front().unwrap();
            let next = match elves.pop_front() {
                Some(elf) => elf,
                None => next_elves.pop_front().unwrap(),
            };

            let num_presents = cur.1 + next.1;
            if num_presents == num_elves {
                println!("Elf in position {} has all the presents", cur.0);
                return;
            }

            next_elves.push_back((cur.0, cur.1 + next.1));
        }

        elves.clear();
        for elf in next_elves.drain(..) {
            elves.push_back(elf);
        }
    }
}
