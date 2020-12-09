pub fn day9(input: &str, preamble: usize) -> i64 {
    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for i in preamble + 1..input.len() {
        if !any_sum(input[i], &input[i - preamble..i]) {
            return input[i];
        }
    }

    unreachable!();
}

pub fn day9_part2(input: &str, preamble: usize) -> i64 {
    let invalid = day9(input, preamble);

    let input = input
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    for start in 0..input.len() {
        for end in start + 1..input.len() {
            let slice = &input[start..end];
            if slice.iter().sum::<i64>() == invalid {
                return slice.iter().max().unwrap() + slice.iter().min().unwrap();
            }
        }
    }

    unreachable!();
}

fn any_sum(number: i64, input: &[i64]) -> bool {
    for i in input {
        for j in input {
            let res = i + j == number;
            if res && i != j {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        day9(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
            5
        ),
        127
    );

    assert_eq!(
        day9_part2(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
            5
        ),
        62,
    );
}
