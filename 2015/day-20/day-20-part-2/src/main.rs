fn main() {
    let target: usize = if cfg!(debug_assertions) {
        150
    } else {
        34000000
    };

    let mut presents = vec![0; target];
    for house in 1..target {
        let mut h = house;
        for _ in 0..50 {
            if h >= presents.len() {
                break;
            }
            presents[h] += house * 11;
            h += house;
        }

        if cfg!(debug_assertions) {
            println!("House {} gets {} presents", house, presents[house]);
        }
        if presents[house] >= target {
            if !cfg!(debug_assertions) {
                println!("House {} gets {} presents", house, presents[house]);
            }
            break;
        }
    }
}
