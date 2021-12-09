#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    NotFound { num_lit: u8 },
}

const NUM_SEGMENTS_1: u8 = 2;
const NUM_SEGMENTS_7: u8 = 3;
const NUM_SEGMENTS_4: u8 = 4;
const NUM_SEGMENTS_235: u8 = 5;

#[derive(Debug, Copy, Clone)]
struct Number([bool; 7], u8);

impl PartialEq for Number {
    fn eq(&self, rhs: &Self) -> bool {
        if self.1 != rhs.1 {
            return false;
        }

        for i in 0..7 {
            if self.0[i] != rhs.0[i] {
                return false;
            }
        }

        true
    }
}
impl Eq for Number {}

impl Number {
    fn num_lit(&self) -> u8 {
        self.1
    }

    fn difference(&self, b: &Self) -> Vec<usize> {
        let mut difference = Vec::new();
        for i in 0..7 {
            if self.0[i] != b.0[i] {
                difference.push(i);
            }
        }
        difference
    }

    fn decode(&self, segments: &[char; 7]) -> usize {
        let mut decoded = [false; 7];
        for i in 0..7 {
            if self.0[i] {
                decoded[(segments[i] as u8 - 'a' as u8) as usize] = true;
            }
        }
        match decoded {
            [true, true, true, false, true, true, true] => 0,
            [false, false, true, false, false, true, false] => 1,
            [true, false, true, true, true, false, true] => 2,
            [true, false, true, true, false, true, true] => 3,
            [false, true, true, true, false, true, false] => 4,
            [true, true, false, true, false, true, true] => 5,
            [true, true, false, true, true, true, true] => 6,
            [true, false, true, false, false, true, false] => 7,
            [true, true, true, true, true, true, true] => 8,
            [true, true, true, true, false, true, true] => 9,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Display(Vec<Number>);

#[derive(Debug)]
struct Displays(Vec<Display>);

impl Display {
    fn find(&self, num_lit: u8) -> Result<Number, Error> {
        for number in &self.0 {
            if number.num_lit() == num_lit {
                return Ok(number.clone());
            }
        }
        Err(Error::NotFound { num_lit: num_lit })
    }

    fn find_possible(&self, num_lit: u8) -> Vec<Number> {
        let mut possible: Vec<Number> = Vec::new();

        for number in &self.0 {
            if number.num_lit() == num_lit {
                if !possible.contains(&number) {
                    possible.push(number.clone());
                }
            }
        }

        possible
    }
}

fn load_input(filename: &str) -> Result<Displays, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let f = File::open(filename).map_err(|e| Error::IO(e))?;

    let lines = BufReader::new(f).lines();

    let mut displays = Displays(Vec::new());

    for line in lines {
        let line = line.map_err(|e| Error::IO(e))?;
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut display = Display(Vec::new());
        for digit in line.replace(" | ", " ").split(" ") {
            let mut number = Number([false; 7], 0);
            let mut num_lit = 0u8;
            for i in 0..7 {
                if digit.contains(('a' as u8 + i) as char) {
                    number.0[i as usize] = true;
                    num_lit += 1;
                }
            }
            number.1 = num_lit;
            display.0.push(number);
        }
        displays.0.push(display);
    }

    Ok(displays)
}

fn find_common(list: &[&Number]) -> Vec<usize> {
    let mut common = Vec::new();
    for i in 0..7 {
        let mut is_common = true;
        for number in list {
            if !number.0[i] {
                is_common = false;
                break;
            }
        }

        if is_common {
            common.push(i);
        }
    }

    common
}

fn main() -> Result<(), Error> {
    let displays = load_input(INPUT_FILE)?;

    let mut total = 0usize;
    for display in &displays.0 {
        let mut segments: [char; 7] = [' '; 7];

        let one = display.find(NUM_SEGMENTS_1)?;
        let four = display.find(NUM_SEGMENTS_4)?;
        let seven = display.find(NUM_SEGMENTS_7)?;
        // let eight = display.find(NUM_SEGMENTS_8)?;

        let difference = one.difference(&seven);
        assert!(difference.len() == 1);
        segments[difference[0]] = 'a';

        let two_three_five = display.find_possible(NUM_SEGMENTS_235);
        assert!(two_three_five.len() == 3);

        let common = find_common(&[
            &two_three_five[0],
            &two_three_five[1],
            &two_three_five[2],
            &four,
        ]);
        assert!(common.len() == 1);
        segments[common[0]] = 'd';

        for i in 0..7 {
            if segments[i] != 'd' && four.0[i] == true && one.0[i] != true {
                // Must be the b segement
                segments[i] = 'b';
            }
        }

        // look for the number which has matches then 1, the only other segment in that number must
        // be the 'g' segment
        for number in &two_three_five {
            let mut matches = true;
            for i in 0..7 {
                if one.0[i] == true && number.0[i] != one.0[i] {
                    matches = false;
                    break;
                }
            }

            if matches {
                // find the missing segment
                for i in 0..7 {
                    if segments[i] == ' ' && number.0[i] != one.0[i] {
                        segments[i] = 'g';
                    }
                }
            }
        }

        // In two_three_five, we can pick out each since we know five must contain b, three must
        // match with one, and two is the other one
        let mut two = None;
        let mut three = None;
        let mut five = None;
        for number in &two_three_five {
            let mut not_three = false;
            let mut is_five = false;
            for i in 0..7 {
                if segments[i] == ' ' && number.0[i] != one.0[i] {
                    not_three = true;
                }
                if segments[i] == 'b' && number.0[i] == true {
                    is_five = true;
                }
            }

            if is_five {
                five = Some(number.clone());
            } else if not_three {
                two = Some(number.clone());
            } else {
                three = Some(number.clone());
            }
        }
        assert!(two.is_some());
        assert!(three.is_some());
        assert!(five.is_some());
        let two = two.unwrap();
        let three = three.unwrap();
        let five = five.unwrap();

        // the missing segment in five is 'f'
        for i in 0..7 {
            if segments[i] == ' ' && five.0[i] {
                segments[i] = 'f';
            }
        }

        // the missing segment in three is 'c'
        for i in 0..7 {
            if segments[i] == ' ' && three.0[i] {
                segments[i] = 'c';
            }
        }

        // the missing segment in two is 'e'
        for i in 0..7 {
            if segments[i] == ' ' && two.0[i] {
                segments[i] = 'e';
            }
        }

        let mut output = 0usize;
        for number in &display.0[10..] {
            let digit = number.decode(&segments);
            output = (output * 10) + digit;
        }
        println!("Output: {}", output);
        total += output;
    }

    println!("Total: {}", total);

    Ok(())
}
