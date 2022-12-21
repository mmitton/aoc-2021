const INPUT_FILE: &str = if cfg!(debug_assertions) {
    "../input-sample.txt"
} else {
    "../input.txt"
};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Monkey {
    Num(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug)]
struct OldNewBTreeMap<K, V>
where
    K: Ord,
    V: Copy,
{
    map: BTreeMap<K, [Option<V>; 2]>,
    len: usize,
}

impl<K, V> OldNewBTreeMap<K, V>
where
    K: Ord,
    V: Copy,
{
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    fn get(&self, k: &K) -> Option<&V> {
        let values = self.map.get(k)?;
        values[0].as_ref()
    }

    fn insert(&mut self, k: K, v: V) {
        if let Some(values) = self.map.get_mut(&k) {
            values[1] = Some(v);
        } else {
            // New item
            let values = [None, Some(v)];
            self.map.insert(k, values);
        }
    }

    fn swap(&mut self) {
        self.len = 0;
        for values in self.map.values_mut() {
            if values[1].is_some() {
                values.swap(0, 1);
                self.len += 1;
            } else {
                values[1] = values[0];
            }
        }
    }

    fn contains_key(&self, k: &K) -> bool {
        if let Some(values) = self.map.get(k) {
            values[0].is_some()
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

fn run(monkeys: &BTreeMap<String, Monkey>, humn: isize) -> OldNewBTreeMap<String, isize> {
    let mut values: OldNewBTreeMap<String, isize> = OldNewBTreeMap::new();
    values.insert("humn".to_owned(), humn);

    while values.len() != monkeys.len() + 1 {
        for (name, op) in monkeys.iter() {
            if values.contains_key(name) {
                continue;
            }
            match op {
                Monkey::Num(n) => {
                    values.insert(name.to_owned(), *n);
                }
                Monkey::Add(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        values.insert(name.to_owned(), *a + *b);
                    }
                }
                Monkey::Sub(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        values.insert(name.to_owned(), *a - *b);
                    }
                }
                Monkey::Mul(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        values.insert(name.to_owned(), *a * *b);
                    }
                }
                Monkey::Div(a, b) => {
                    if let (Some(a), Some(b)) = (values.get(a), values.get(b)) {
                        values.insert(name.to_owned(), *a / *b);
                    }
                }
            }
        }

        values.swap();
    }

    values
}

fn main() {
    let lines: Vec<String> = {
        let file = File::open(INPUT_FILE).expect("Cannot open input file");
        BufReader::new(file).lines().flatten().collect()
    };

    let mut monkeys: BTreeMap<String, Monkey> = BTreeMap::new();
    let mut root = ("".to_string(), "".to_string());
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let name = line[0..4].to_string();
        let op = &line[6..];
        match name.as_str() {
            "root" => {
                let (a, b) = op.split_once(" + ").unwrap();
                root = (a.to_owned(), b.to_owned());
            }
            "humn" => {
                // monkeys.insert(name.to_owned(), Monkey::Num(0));
            }
            _ => {
                if let Some((a, b)) = op.split_once(" + ") {
                    monkeys.insert(name, Monkey::Add(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" - ") {
                    monkeys.insert(name, Monkey::Sub(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" * ") {
                    monkeys.insert(name, Monkey::Mul(a.to_string(), b.to_string()));
                } else if let Some((a, b)) = op.split_once(" / ") {
                    monkeys.insert(name, Monkey::Div(a.to_string(), b.to_string()));
                } else {
                    monkeys.insert(name, Monkey::Num(op.parse().unwrap()));
                }
            }
        }
    }

    let mut h = 0;
    let values = run(&monkeys, h);
    let (a, b) = (*values.get(&root.0).unwrap(), *values.get(&root.1).unwrap());
    let initial = a < b;
    let mut scale = 1;
    while scale * 10 < (a - b).abs() {
        scale *= 10;
    }

    loop {
        let cur_h = h + scale;
        let values = run(&monkeys, cur_h);
        let (a, b) = (*values.get(&root.0).unwrap(), *values.get(&root.1).unwrap());
        if a == b {
            println!("Done!  ans:{cur_h}");
            break;
        }
        println!("cur_h:{cur_h}  scale:{scale}  a:{a}  b:{b}  diff:{}", a - b);
        if (initial && a > b) || (!initial && a < b) {
            // Lower the scale and try again
            scale /= 10;
        } else {
            h += scale;
        }
    }
}
