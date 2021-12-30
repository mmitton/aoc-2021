use md5;

fn main() {
    let input = if cfg!(debug_assertions) {
        "abc"
    } else {
        "reyedfim"
    };

    let initial_input = input.as_bytes();
    let mut input = Vec::new();

    let mut password = Vec::new();

    for i in 0..u32::MAX {
        input.clear();
        input.extend_from_slice(&initial_input);
        input.extend_from_slice(i.to_string().as_bytes());
        let result = md5::compute(&input);

        if result[0] == 0 && result[1] == 0 && result[2] >> 4 == 0 {
            password.push(format!("{:x}", result[2] & 0xF));
            println!(
                "Found next char {} of password at {}.  {}",
                password.len(),
                i,
                password.join("")
            );

            if password.len() == 8 {
                break;
            }
        }
    }
}
