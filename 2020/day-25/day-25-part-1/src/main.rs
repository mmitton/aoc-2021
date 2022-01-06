fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut v = 1;
    for _ in 0..loop_size {
        v *= subject_number;
        v = v % 20201227;
    }

    v
}

fn find_loop(public: usize) -> usize {
    for loop_size in 1..usize::MAX {
        if transform(7, loop_size) == public {
            return loop_size;
        }
    }

    panic!();
}

fn main() {
    let (card_public, door_public) = if cfg!(debug_assertions) {
        (5764801, 17807724)
    } else {
        (9232416, 14144084)
    };

    let mut card_loop = None;
    let mut door_loop = None;

    let mut v = 1;
    let mut loops = 0;
    while card_loop.is_none() || door_loop.is_none() {
        v *= 7;
        v = v % 20201227;
        loops += 1;

        if card_loop.is_none() && v == card_public {
            card_loop = Some(loops);
        }
        if door_loop.is_none() && v == door_public {
            door_loop = Some(loops);
        }
    }
    let card_loop = card_loop.unwrap();
    let door_loop = door_loop.unwrap();

    println!(
        "Card Public: {}  Loop: {}  Key: {}",
        card_public,
        card_loop,
        transform(door_public, card_loop)
    );
    println!(
        "Door Public: {}  Loop: {}  Key: {}",
        door_public,
        door_loop,
        transform(card_public, door_loop)
    );
}
