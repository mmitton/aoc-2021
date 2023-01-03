struct Test([u8; 6]);

impl Test {
    fn new(mut num: usize) -> Self {
        let mut v = Vec::new();
        let mut scale = 100000;
        while scale > 0 {
            v.push((num / scale) as u8);
            num %= scale;
            scale /= 10;
        }

        Self(v.as_slice().try_into().unwrap())
    }

    fn is_valid(&self) -> bool {
        let mut has_dup = false;
        let mut is_assending = true;
        for a in 0..5 {
            if self.0[a] == self.0[a + 1] {
                has_dup = true;
            }
            if self.0[a] > self.0[a + 1] {
                is_assending = false;
            }
        }

        has_dup && is_assending
    }
}

fn main() {
    let low = 156218;
    let high = 652527;
    let mut ans = 0;
    for num in low..=high {
        let test = Test::new(num);
        if test.is_valid() {
            ans += 1;
        }
    }

    println!("ans: {ans}");
}
