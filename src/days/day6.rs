use std::collections::{HashMap, HashSet};

pub fn day6(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|group| {
            let mut set = HashSet::new();

            group
                .chars()
                .filter(|ch| ch.is_ascii_alphabetic())
                .for_each(|ch| {
                    set.insert(ch);
                });

            set.len() as i64
        })
        .sum::<i64>()
}

pub fn day6_part2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::new();

            for ch in group.chars().filter(|ch| ch.is_ascii_alphabetic()) {
                *map.entry(ch).or_insert(0) += 1;
            }

            map.values()
                .filter(|n| **n == group.lines().count())
                .count() as i64
        })
        .sum::<i64>()
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(day6("abcx\nabcy\nabcz\n"), 6);
    assert_eq!(day6("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n"), 11);
    assert_eq!(
        day6_part2("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n"),
        6
    );
}
