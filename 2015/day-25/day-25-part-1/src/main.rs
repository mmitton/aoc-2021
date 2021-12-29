fn calc(r: usize, c: usize, prev: Option<usize>) -> usize {
    match prev {
        Some(prev) => (prev * 252533) % 33554393,
        _ => {
            let mut cr = 1;
            let mut cc = 1;
            let mut code = 20151125;
            while cr != r || cc != c {
                code = (code * 252533) % 33554393;
                if cr == 1 {
                    cr = cc + 1;
                    cc = 1;
                } else {
                    cr -= 1;
                    cc += 1;
                }
            }

            code
        }
    }
}

fn main() {
    for r in 1..7 {
        for c in 1..7 {
            print!("{:<10}", calc(r, c, None));
        }
        println!();
    }

    println!();
    let row = 2978;
    let column = 3083;
    println!(
        "row: {}, column: {} = {}",
        row,
        column,
        calc(row, column, None)
    );
}
