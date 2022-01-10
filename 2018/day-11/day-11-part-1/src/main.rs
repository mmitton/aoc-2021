fn get_power_level(serial: usize, x: usize, y: usize) -> isize {
    let serial = serial as isize;
    let x = x as isize;
    let y = y as isize;

    let rack_id = x + 10;
    let rack_power_level = ((rack_id * y) + serial) * rack_id;
    let power_level = (rack_power_level / 100) % 10;
    power_level - 5
}

fn get_block_power_level(serial: usize, x: usize, y: usize) -> isize {
    let mut block_power_level = 0;
    for x in x..x + 3 {
        for y in y..y + 3 {
            block_power_level += get_power_level(serial, x, y);
        }
    }
    block_power_level
}

fn find_best_block(
    serial: usize,
    block_x_size: usize,
    block_y_size: usize,
    total_x_size: usize,
    total_y_size: usize,
) -> (usize, usize, isize) {
    use std::collections::BTreeMap;
    let mut power_levels = BTreeMap::new();

    for x in 1..=total_x_size {
        for y in 1..=total_y_size {
            power_levels.insert((x, y), get_power_level(serial, x, y));
        }
    }

    let mut max_block_power_level = isize::MIN;
    let mut max_x = 0;
    let mut max_y = 0;

    for sx in 1..=total_x_size - block_x_size + 1 {
        for sy in 1..=total_y_size - block_y_size + 1 {
            let mut block_power_level = 0;
            for x in sx..sx + block_x_size {
                for y in sy..sy + block_y_size {
                    block_power_level += get_power_level(serial, x, y);
                }
            }
            if block_power_level > max_block_power_level {
                max_block_power_level = block_power_level;
                max_x = sx;
                max_y = sy;
            }
        }
    }

    (max_x, max_y, max_block_power_level)
}

fn main() {
    assert!(get_power_level(8, 3, 5) == 4);
    assert!(get_power_level(57, 122, 79) == -5);
    assert!(get_power_level(39, 217, 196) == 0);
    assert!(get_power_level(71, 101, 153) == 4);

    assert!(get_block_power_level(18, 33, 45) == 29);
    assert!(get_block_power_level(42, 21, 61) == 30);

    let (x, y, power) = find_best_block(2866, 3, 3, 300, 300);
    println!("{},{}  =  {}", x, y, power);
}
