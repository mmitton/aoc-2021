const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::{BTreeMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Reaction {
    output: (String, usize),
    need: Vec<(String, usize)>,
}

fn process(reactions: &BTreeMap<String, Reaction>) {
    let mut ore = 0;
    struct Inventory {
        built: usize,
        consumed: usize,
    }
    let mut inventory: BTreeMap<String, Inventory> = BTreeMap::new();
    let mut work = VecDeque::new();
    work.push_front(("FUEL".to_string(), 1));
    while let Some(need) = work.pop_front() {
        if need.0 == "ORE" {
            ore += need.1;
        } else {
            let inv = inventory.entry(need.0.clone()).or_insert(Inventory {
                built: 0,
                consumed: 0,
            });
            if inv.built - inv.consumed >= need.1 {
                inv.consumed += need.1;
            } else {
                let still_need = need.1 - (inv.built - inv.consumed);
                let reaction = reactions.get(&need.0).unwrap();
                let reactions_needed = (still_need + reaction.output.1 - 1) / reaction.output.1;
                inv.built += reactions_needed * reaction.output.1;
                inv.consumed += need.1;

                for need in reaction.need.iter() {
                    work.push_back((need.0.clone(), need.1 * reactions_needed));
                }
            }
        }
    }
    println!("ore needed for 1 FUEL: {ore}");
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut reactions: BTreeMap<String, Reaction> = BTreeMap::new();
    for line in lines.iter() {
        if line.is_empty() {
            process(&reactions);
            reactions.clear();
            continue;
        }

        macro_rules! parse {
            ($str:expr) => {{
                let (num, name) = $str.split_once(" ").unwrap();
                let num: usize = num.parse().unwrap();
                (name.to_string(), num)
            }};
        }

        let (needed, output) = line.split_once(" => ").unwrap();
        let output = parse!(output);
        let mut need = Vec::new();
        for n in needed.split(", ") {
            need.push(parse!(n));
        }
        reactions.insert(output.0.clone(), Reaction { output, need });
    }
    process(&reactions);
}
