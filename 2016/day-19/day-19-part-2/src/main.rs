struct Elf {
    num: usize,
    presents: usize,
    next: usize,
    prev: usize,
}

fn main() {
    let num_elves = if cfg!(debug_assertions) { 5 } else { 3017957 };

    let mut elves = Vec::new();
    for elf in 0..num_elves as usize {
        let elf = Elf {
            num: elf + 1,
            presents: 1,
            next: if elf == num_elves - 1 { 0 } else { elf + 1 },
            prev: if elf == 0 { num_elves - 1 } else { elf - 1 },
        };
        elves.push(elf);
    }
    let mut cur = 0;
    let mut across = num_elves / 2;
    let mut elves_left = num_elves;
    loop {
        if cfg!(debug_assertions) {
            println!(
                "Elf {} takes {} presents from {}",
                elves[cur].num, elves[across].presents, elves[across].num
            );
        }
        elves[cur].presents += elves[across].presents;
        elves[across].presents = 0;

        // Remove across
        let prev = elves[across].prev;
        let next = elves[across].next;
        elves[prev].next = next;
        elves[next].prev = prev;

        elves_left -= 1;
        if elves_left % 2 == 0 {
            across = elves[next].next;
        } else {
            across = next;
        }

        cur = elves[cur].next;

        if elves_left == 1 {
            println!("Elf in position {} takes all the presents", elves[cur].num);
            return;
        }
    }
}
