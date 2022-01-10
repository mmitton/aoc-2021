#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<(Vec<isize>, Vec<u8>), Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();
    let mut initial_state = Vec::new();
    let mut on_states = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim().to_string();
        if line == "" {
            continue;
        }

        fn parse(s: &str) -> Vec<bool> {
            let mut v = Vec::new();
            for c in s.chars() {
                let b = match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                };
                v.push(b);
            }
            v
        }

        if line.starts_with("initial state: ") {
            initial_state.clear();
            for (i, b) in parse(&line["initial state: ".len()..]).iter().enumerate() {
                if *b {
                    initial_state.push(i as isize)
                }
            }
        } else {
            if line.ends_with("#") {
                let mut n = 0u8;
                for b in parse(&line[..5]).iter() {
                    n <<= 1;
                    if *b {
                        n |= 1;
                    }
                }
                on_states.push(n);
            }
        }
    }

    Ok((initial_state, on_states))
}

fn format_full(v: &Vec<isize>) -> String {
    let mut s = String::new();

    let min_x = v[0] - 2;
    let max_x = v[v.len() - 1] + 2;

    for i in min_x..=max_x {
        s.push(if v.contains(&i) { '#' } else { '.' });
    }

    s
}

fn format(v: u8) -> String {
    let mut s = String::new();

    for i in (0..5).rev() {
        s.push(if (v >> i) & 1 == 1 { '#' } else { '.' });
    }

    s
}

fn next_gen(states: &mut Vec<Vec<isize>>, cur: usize, on_states: &Vec<u8>) {
    let next = 1 - cur;

    macro_rules! get {
        ($i:expr) => {
            if states[cur].contains(&$i) {
                1
            } else {
                0
            }
        };
    }

    let min_x = states[cur][0] - 2;
    let max_x = states[cur][states[cur].len() - 1] + 2;

    states[next].clear();

    for i in min_x..=max_x {
        let mut c = 0u8;

        c = c << 1 | get!(i - 2);
        c = c << 1 | get!(i - 1);
        c = c << 1 | get!(i);
        c = c << 1 | get!(i + 1);
        c = c << 1 | get!(i + 2);

        if on_states.contains(&c) {
            states[next].push(i);
        }
    }
}

fn main() -> Result<(), Error> {
    let (initial_state, on_states) = load_input(INPUT_FILE)?;

    println!("initial_state: {}", format_full(&initial_state));
    for on_state in &on_states {
        println!("on_state: {}", format(*on_state));
    }

    let mut states = vec![initial_state.clone(), Vec::new()];
    let mut cur = 0;
    println!("0: {}", format_full(&states[cur]));
    for gen in 1..=20 {
        next_gen(&mut states, cur, &on_states);
        cur = 1 - cur;
        println!("{}: {}", gen, format_full(&states[cur]));
    }

    let mut sum = 0isize;
    for i in &states[cur] {
        sum += i;
    }

    println!("Sum: {}", sum);

    Ok(())
}
