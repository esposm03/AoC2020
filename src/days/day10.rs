use std::collections::HashMap;

pub fn day10(input: &str) -> i64 {
    let mut input = input
        .lines()
        .map(|i| i.parse::<i64>().expect(&i.to_string()))
        .collect::<Vec<_>>();
    input.sort_unstable();

    let mut last_elem = 0;
    let mut num1 = 0;
    let mut num3 = 1;
    for i in input {
        if i == last_elem + 1 {
            num1 += 1
        }
        if i == last_elem + 3 {
            num3 += 1
        }
        last_elem = i;
    }

    num1 * num3
}

pub fn day10_part2(input: &str) -> i64 {
    // I copied this code from [here](https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/10.rs),
    // as sadly I couldn't do it :(

    let mut v = input
        .lines()
        .map(|i| i.parse::<i64>().expect(&i.to_string()))
        .collect::<Vec<_>>();
    v.sort_unstable();

    let mut dp = HashMap::new();
    dp.insert(0, 1);
    for i in &v {
        let ans = dp.get(&(i - 1)).unwrap_or(&0)
            + dp.get(&(i - 2)).unwrap_or(&0)
            + dp.get(&(i - 3)).unwrap_or(&0);
        dp.insert(*i, ans);
    }
    dp[v.last().unwrap()]
}

#[cfg(test)]
#[test]
fn test() {
    let input1 = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n";
    let input2 = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n";

    assert_eq!(35, day10(input1));
    assert_eq!(220, day10(input2));

    assert_eq!(8, day10_part2(input1));
    assert_eq!(19208, day10_part2(input2));
}
