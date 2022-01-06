fn move_cups(cups: &mut Vec<u8>, buffer: &mut Vec<u8>, cur: usize) -> usize {
    let cur_num = cups[cur];
    println!("cups: {:?}  current: {}", cups, cur_num);
    let mut idx = cur + 1;
    for _ in 0..3 {
        if idx >= cups.len() {
            idx = 0;
        }
        buffer.push(cups.remove(idx));
    }
    println!("pickup: {:?}", buffer);

    let mut next_num = cur_num;
    loop {
        next_num -= 1;
        if next_num == 0 {
            next_num = 9;
        }
        if cups.contains(&next_num) {
            break;
        }
    }

    println!("destination: {}", next_num);
    let mut dest_idx = 0;
    for i in 0..cups.len() {
        if cups[i] == next_num {
            dest_idx = i;
        }
    }
    while buffer.len() > 0 {
        cups.insert(dest_idx + 1, buffer.remove(buffer.len() - 1));
    }
    println!();

    let mut next = cur;
    if dest_idx < next {
        next += 3;
    }
    if next >= cups.len() {
        0
    } else {
        (next + 1) % cups.len()
    }
}

fn main() {
    let input = if cfg!(debug_assertions) {
        "389125467"
    } else {
        "739862541"
    };

    let mut cups: Vec<u8> = Vec::new();
    for c in input.chars() {
        cups.push(c as u8 - '0' as u8);
    }
    let mut buffer: Vec<u8> = Vec::new();

    let mut cur = 0;
    for _ in 0..100 {
        cur = move_cups(&mut cups, &mut buffer, cur);
    }

    println!("Cups: {:?}", cups);
    print!("Answer: ");
    for i in 0..cups.len() {
        if cups[i] == 1 {
            for i in i + 1..i + cups.len() {
                print!("{}", cups[i % cups.len()]);
            }
        }
    }
    println!();
}
