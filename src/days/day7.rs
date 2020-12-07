use std::collections::HashMap;

pub fn day7(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|ln| parsing::line(ln).unwrap().1)
        .collect::<HashMap<_, _>>();

    let mut count = 0;
    for container in map.keys() {
        if has_gold(&map, container) {
            count += 1;
        }
    }

    count
}

fn has_gold(map: &HashMap<&str, Vec<&str>>, name: &str) -> bool {
    map.get(name)
        .unwrap()
        .iter()
        .any(|i| *i == "shiny gold" || has_gold(map, i))
}

mod parsing {
    use nom::{
        bytes::complete as bytes, character::complete as character, combinator, multi, IResult,
    };

    pub fn line(i: &str) -> IResult<&str, (&str, Vec<&str>)> {
        let (i, container) = bag(i)?;
        let (i, _) = bytes::tag(" bags contain ")(i)?;
        let (i, contents) = nom::branch::alt((multi::many1(content), no_content))(i)?;

        Ok((i, (container, contents)))
    }

    fn no_content(i: &str) -> IResult<&str, Vec<&str>> {
        let (i, _) = bytes::tag("no other bags")(i)?;
        Ok((i, vec![]))
    }

    fn content(i: &str) -> IResult<&str, &str> {
        let (i, _) = character::space0(i)?;
        let (i, _) = character::digit1(i)?;
        let (i, _) = bytes::tag(" ")(i)?;
        let (i, bag) = bag(i)?;
        let (i, _) = bytes::tag(" bag")(i)?;

        let (i, _) = combinator::opt(bytes::tag("s"))(i)?;
        let (i, _) = combinator::opt(bytes::tag(", "))(i)?;

        Ok((i, bag))
    }

    fn bag(i: &str) -> IResult<&str, &str> {
        let (i, _) = character::space0(i)?;
        let start = i;

        let (i, _) = character::alpha1(i)?;
        let (i, _) = character::space0(i)?;
        let (i, _) = character::alpha1(i)?;

        Ok((i, &start[..(start.len() - i.len())]))
    }

    #[test]
    #[cfg(test)]
    fn parsing_test() {
        assert_eq!(
            line("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap()
                .1,
            ("light red", vec!["bright white", "muted yellow"])
        );
    }
}

#[cfg(test)]
#[test]
fn test() {
    let mut map = HashMap::new();
    map.insert("light red", vec!["bright white", "muted yellow"]);
    map.insert("dark orange", vec!["bright white", "muted yellow"]);
    map.insert("bright white", vec!["shiny gold"]);

    assert!(has_gold(&map, "bright white"));
    assert!(has_gold(&map, "light red"));
}
