fn main() {
    let (target_len, data) = if cfg!(debug_assertions) {
        (20, "10000")
    } else {
        (35651584, "10010000000110000")
    };
    let mut data = data.to_string();

    while data.len() < target_len {
        let chars: Vec<char> = data.chars().collect();
        data.push('0');
        for c in chars.iter().rev() {
            match c {
                '1' => data.push('0'),
                '0' => data.push('1'),
                _ => unreachable!(),
            }
        }
    }
    let data = data[0..target_len].to_string();
    let mut checksum = data.clone();
    loop {
        let mut new_checksum = String::new();
        for i in (0..checksum.len()).step_by(2) {
            match &checksum[i..i + 2] {
                "00" | "11" => new_checksum.push('1'),
                _ => new_checksum.push('0'),
            }
        }
        checksum = new_checksum;

        if checksum.len() % 2 == 1 {
            break;
        }
    }

    println!("Checksum: {}, {}", checksum.len(), checksum);
}
