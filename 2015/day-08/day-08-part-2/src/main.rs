#[cfg(debug_assertions)]
const INPUT_FILE: &str = "../input-sample.txt";
#[cfg(not(debug_assertions))]
const INPUT_FILE: &str = "../input.txt";

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    StringError(std::str::Utf8Error),
    NAN(std::num::ParseIntError),
}

fn load_input(filename: &str) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(filename).map_err(|e| Error::IO(e))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| Error::IO(e))?;

    let mut inputs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    let mut in_string = false;
    let mut string = Vec::new();
    let mut i = 0;
    let mut string_start = 0;
    loop {
        let c = buf[i] as char;
        if !in_string {
            if c == '"' {
                in_string = true;
                string.clear();
                string_start = i;
            }
        } else {
            // In string
            if c == '"' {
                inputs.push((buf[string_start..i + 1].to_vec(), string.clone()));
                in_string = false;
            } else if c == '\\' {
                i += 1;
                match buf[i] as char {
                    'x' => {
                        // Grab the next two chars, they are hex of ascii
                        let hex: &str = std::str::from_utf8(&buf[i + 1..i + 2])
                            .map_err(|e| Error::StringError(e))?;
                        let num = u8::from_str_radix(&hex, 16).map_err(|e| Error::NAN(e))?;
                        println!("hex: {}  num:{}", hex, num);
                        string.push(num);
                        i += 2;
                    }
                    _ => string.push(buf[i]),
                }
            } else {
                string.push(buf[i]);
            }
        }
        i += 1;
        if i == buf.len() {
            break;
        }
    }

    Ok(inputs)
}

fn encode(buf: &Vec<u8>) -> Vec<u8> {
    let mut encoded: Vec<u8> = Vec::new();

    encoded.push('"' as u8);
    for b in buf {
        if *b == '"' as u8 {
            encoded.push('\\' as u8);
        }
        if *b == '\\' as u8 {
            encoded.push('\\' as u8);
        }

        encoded.push(*b);
    }
    encoded.push('"' as u8);

    encoded
}

fn main() -> Result<(), Error> {
    let inputs = load_input(INPUT_FILE)?;

    let mut total_encoded = 0;
    let mut total_in_mem = 0;
    for input in &inputs {
        let encoded = encode(&input.0);
        println!("{} {} {}", input.0.len(), input.1.len(), encoded.len());
        total_encoded += encoded.len();
        total_in_mem += input.0.len();
    }

    println!(
        "total_encoded:{}  total_in_mem:{}   Answer: {}",
        total_encoded,
        total_in_mem,
        total_encoded - total_in_mem
    );
    Ok(())
}
