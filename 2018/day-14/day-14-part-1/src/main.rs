fn answer(iters: usize) -> String {
    let mut scores = vec![3, 7];
    let mut elves = vec![0, 1];

    for _ in 0..iters + 10 {
        let new_score = scores[elves[0]] + scores[elves[1]];
        if new_score > 9 {
            scores.push(new_score / 10);
            scores.push(new_score % 10);
        } else {
            scores.push(new_score);
        }

        for i in 0..elves.len() {
            elves[i] = (elves[i] + 1 + scores[elves[i]]) % scores.len();
        }
    }

    let mut answer = String::new();
    for i in 0..10 {
        answer.push_str(format!("{}", scores[iters + i]).as_str())
    }

    answer
}

fn main() {
    assert!(answer(9) == "5158916779");
    assert!(answer(5) == "0124515891");
    assert!(answer(18) == "9251071085");
    assert!(answer(2018) == "5941429882");

    println!("Answer: {}", answer(513401));
}
