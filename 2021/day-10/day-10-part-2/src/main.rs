#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
}

fn load_input(filename: &str) -> Result<Vec<Vec<char>>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut ret = Vec::new();

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        ret.push(line.chars().collect::<Vec<char>>());
    }

    Ok(ret)
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut answers = Vec::new();
    'input_loop: for input in inputs {
        let mut stack = Vec::new();
        for c in input {
            let lookfor = match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    None
                }
                ')' => Some('('),
                ']' => Some('['),
                '}' => Some('{'),
                '>' => Some('<'),
                _ => panic!("unexpected input '{}'", c),
            };

            if let Some(lookfor) = lookfor {
                let last = stack.pop().expect("Stack empty");
                if last != lookfor {
                    continue 'input_loop;
                }
            }
        }

        let mut my_points = 0usize;
        print!("{:?}  ", stack);
        while stack.len() != 0 {
            my_points *= 5;
            let last = stack.pop().expect("Stack empty");
            match last {
                '(' => {
                    my_points += 1;
                    print!(")");
                }
                '[' => {
                    my_points += 2;
                    print!("]");
                }
                '{' => {
                    my_points += 3;
                    print!("}}");
                }
                '<' => {
                    my_points += 4;
                    print!(">");
                }
                _ => panic!("expected data on stack '{}'", last),
            }
        }

        println!("  {}", my_points);
        answers.push(my_points);
    }

    answers.sort();
    let answer = answers[answers.len() / 2];

    println!("Answer: {} .. {:?}", answer, answers);
    Ok(())
}
