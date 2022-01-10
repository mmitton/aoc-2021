fn answer(mut t: usize) -> usize {
    let mut scores = vec![3, 7];
    let mut elves = vec![0, 1];
    let mut target = Vec::new();
    while t > 0 {
        target.insert(0, t % 10);
        t /= 10;
    }

    for _ in 0..usize::MAX {
        let new_score = scores[elves[0]] + scores[elves[1]];
        if new_score > 9 {
            scores.push(new_score / 10);
            scores.push(new_score % 10);
        } else {
            scores.push(new_score);
        }

        if scores.len() > target.len() + 1 {
            if scores[scores.len() - target.len()..].eq(&target) {
                println!("Target: {:?}  Scores: {:?}", target, scores);
                return scores.len() - target.len();
            } else if scores[scores.len() - target.len() - 1..scores.len() - 1].eq(&target) {
                return scores.len() - target.len() - 1;
            }
        }

        for i in 0..elves.len() {
            elves[i] = (elves[i] + 1 + scores[elves[i]]) % scores.len();
        }
    }

    panic!();
}

fn main() {
    let ans = answer(9);
    assert!(ans == 13, "{} != {}", ans, 13);
    let ans = answer(5);
    assert!(ans == 9, "{} != {}", ans, 9);
    let ans = answer(12);
    assert!(ans == 6, "{} != {}", ans, 6);
    let ans = answer(24);
    assert!(ans == 7, "{} != {}", ans, 7);
    let ans = answer(45);
    assert!(ans == 8, "{} != {}", ans, 8);
    let ans = answer(51);
    assert!(ans == 9, "{} != {}", ans, 9);

    println!("Answer: {}", answer(513401));
}
