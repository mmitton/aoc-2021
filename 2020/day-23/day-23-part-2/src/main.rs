// const CUPS: usize = 9;
// const ITERS: usize = 100;
const CUPS: usize = 1_000_000;
const ITERS: usize = 10_000_000;

fn main() {
    let input = if cfg!(debug_assertions) {
        "389125467"
    } else {
        "739862541"
    };

    let mut cups: Vec<usize> = Vec::with_capacity(CUPS);
    for i in 0..CUPS {
        cups.push(i + 1);
    }
    let mut last_idx = 0;
    for c in input.chars() {
        let c = (c as u8 - '0' as u8) as usize;
        cups[last_idx] = c;
        last_idx = c;
    }
    cups[last_idx] = 10;
    cups[CUPS - 1] = 0;

    let mut cur = cups[0];
    let mut holding: [usize; 3] = [0; 3];

    for _ in 0..ITERS {
        // "copy" 3 holding
        holding[0] = cups[cur];
        holding[1] = cups[holding[0]];
        holding[2] = cups[holding[1]];

        // find destination
        let mut dest = if cur == 0 { CUPS - 1 } else { cur - 1 };
        while holding.contains(&dest) {
            dest = dest - 1;
        }

        // update
        let tmp = cups[dest];
        cups[dest] = holding[0];
        cups[cur] = cups[holding[2]];
        cups[holding[2]] = tmp;

        cur = cups[cur];
    }

    println!(
        "Answer: {} * {} = {}",
        cups[1],
        cups[cups[1]],
        cups[1] * cups[cups[1]]
    );
}
