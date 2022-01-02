#[cfg(debug_assertions)]
const INPUT: &str = "abc";
#[cfg(not(debug_assertions))]
const INPUT: &str = "qzyelonm";

#[derive(Debug)]
struct Hash {
    digits: Vec<u8>,
    threes: Vec<u8>,
    fives: Vec<u8>,
}

impl Hash {
    fn new(digits: Vec<u8>) -> Self {
        let mut hash = Hash {
            digits: digits,
            threes: Vec::new(),
            fives: Vec::new(),
        };

        let mut i = 0;
        while i < hash.digits.len() - 2 {
            if hash.digits[i] == hash.digits[i + 1] && hash.digits[i] == hash.digits[i + 2] {
                if hash.threes.len() == 0 {
                    // !hash.threes.contains(&hash.digits[i]) {
                    hash.threes.push(hash.digits[i]);
                }
                if i < hash.digits.len() - 4 {
                    if hash.digits[i] == hash.digits[i + 3] && hash.digits[i] == hash.digits[i + 4]
                    {
                        if !hash.fives.contains(&hash.digits[i]) {
                            hash.fives.push(hash.digits[i]);
                        }
                        i += 2;
                    }
                }

                i += 3;
            } else {
                i += 1;
            }
        }

        hash
    }
}

struct OneTimePad {
    hashes: Vec<Hash>,
    initial_input: &'static [u8],
    input: Vec<u8>,
}

impl OneTimePad {
    fn new() -> Self {
        Self {
            hashes: Vec::new(),
            initial_input: INPUT.as_bytes(),
            input: Vec::new(),
        }
    }

    fn get_hash(&mut self, n: usize) -> &Hash {
        while self.hashes.len() <= n {
            self.input.clear();
            self.input.extend_from_slice(&self.initial_input);
            self.input
                .extend_from_slice(self.hashes.len().to_string().as_bytes());

            let mut result: Vec<u8> = Vec::new();
            for _ in 0..=2016 {
                let md5 = md5::compute(&self.input);
                let next = format!("{:?}", md5);
                result = md5::compute(&self.input).into_iter().collect();
                self.input.clear();
                self.input.extend_from_slice(&next.as_bytes());
            }

            let mut hash = Vec::new();
            for r in &result {
                hash.push(r >> 4);
                hash.push(r & 0xF);
            }

            self.hashes.push(Hash::new(hash));
        }

        &self.hashes[n]
    }
}

fn main() {
    let mut generator = OneTimePad::new();

    let mut keys = Vec::new();

    'search_loop: for i in 0..usize::MAX {
        let threes = generator.get_hash(i).threes.clone();

        if threes.len() > 0 {
            for n in &threes {
                for j in i + 1..=i + 1000 {
                    let hash = generator.get_hash(j);
                    if hash.fives.contains(n) {
                        keys.push(i);
                        println!("{}: {}", keys.len(), i);
                        if keys.len() == 64 {
                            println!("Answer: {}", i);
                            return;
                        }
                        continue 'search_loop;
                    }
                }
            }
        }
    }
}
