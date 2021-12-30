use md5;

fn main() {
    let input = if cfg!(debug_assertions) {
        "abc"
    } else {
        "reyedfim"
    };

    let initial_input = input.as_bytes();
    let mut input = Vec::new();

    let mut password = vec!['_'; 8];

    for i in 0..u32::MAX {
        input.clear();
        input.extend_from_slice(&initial_input);
        input.extend_from_slice(i.to_string().as_bytes());
        let result = md5::compute(&input);

        if result[0] == 0 && result[1] == 0 && result[2] >> 4 == 0 {
            let pos = result[2] as usize & 0xF;
            if pos < 8 && password[pos] == '_' {
                password[pos] = format!("{:x}", result[3] >> 4).chars().nth(0).unwrap();
                println!(
                    "Found next char {} of password at {}.  {}",
                    password.len(),
                    i,
                    password.iter().collect::<String>()
                );

                if !password.contains(&'_') {
                    break;
                }
            }
        }
    }
}
