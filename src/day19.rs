use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug)]
enum XmasProperty {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Operation {
    GreaterThan(XmasProperty, u32),
    LessThan(XmasProperty, u32),
}

impl Operation {
    fn passes(&self, part: &Part) -> bool {
        match self {
            GreaterThan(property, value) => match property {
                X => part.x > *value,
                M => part.m > *value,
                A => part.a > *value,
                S => part.s > *value,
            },
            LessThan(property, value) => match property {
                X => part.x < *value,
                M => part.m < *value,
                A => part.a < *value,
                S => part.s < *value,
            },
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    test: Option<Operation>,
    destination_workflow: &'a str,
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

use Operation::*;
use XmasProperty::*;

impl Workflow<'_> {
    fn next(&self, part: &Part) -> &'_ str {
        let rule = self.rules.iter().find(|rule| match &rule.test {
            None => return true,
            Some(op) => {
                if op.passes(part) {
                    return true;
                } else {
                    return false;
                }
            }
        });
        rule.expect("should have matched something")
            .destination_workflow
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, alpha1, newline},
        combinator::opt,
        multi::{many1, separated_list1},
        sequence::{delimited, preceded, separated_pair},
        IResult,
    };

    // px{a<2006:qkq,m>2090:A,rfg}
    //
    // {x=787,m=2655,a=1222,s=2876}
    pub fn all(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
        let (input, (workflows, parts)) = separated_pair(workflows, many1(newline), parts)(input)?;
        Ok((input, (workflows, parts)))
    }

    fn parts(input: &str) -> IResult<&str, Vec<Part>> {
        let (input, parts) = separated_list1(newline, part)(input)?;
        Ok((input, parts))
    }

    fn part(input: &str) -> IResult<&str, Part> {
        let (input, details) = delimited(
            tag("{"),
            separated_list1(tag(","), separated_pair(alpha1, tag("="), complete::u32)),
            tag("}"),
        )(input)?;
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for (name, value) in details {
            match name {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => unreachable!("unexpected parse!"),
            }
        }
        Ok((input, part))
    }

    pub fn workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
        let (input, workflows) = separated_list1(newline, workflow)(input)?;
        Ok((input, workflows))
    }

    fn workflow(input: &str) -> IResult<&str, Workflow> {
        // px{a<2006:qkq,m>2090:A,rfg}
        let (input, name) = alpha1(input)?;
        let (input, rules) = delimited(tag("{"), rules, tag("}"))(input)?;
        Ok((input, Workflow { name, rules }))
    }

    fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
        let (input, rules) = separated_list1(tag(","), rule)(input)?;
        Ok((input, rules))
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        let (input, xmas) = alpha1(input)?;
        let (input, test) = opt(alt((tag("<"), tag(">"))))(input)?;
        let (input, amount) = opt(complete::u32)(input)?;
        let (input, mut destination) = opt(preceded(tag(":"), alpha1))(input)?;

        let property;

        if destination.is_some() {
            property = Some(match xmas {
                "x" => X,
                "m" => M,
                "a" => A,
                "s" => S,
                _ => unreachable!("should have been handled above"),
            });
        } else {
            // This is the last rule in the workflow
            property = None;
            destination = Some(xmas);
        }

        let test = match test {
            None => None,
            Some(op) => match op {
                "<" => Some(LessThan(
                    property.expect("should always have a property here"),
                    amount.expect("AoC parsing should always work"),
                )),
                ">" => Some(GreaterThan(
                    property.expect("should always have a property here"),
                    amount.expect("AoC parsing should always work"),
                )),
                _ => unreachable!("Should only have > and <"),
            },
        };

        Ok((
            input,
            Rule {
                test,
                destination_workflow: destination.expect("should parse a destination"),
            },
        ))
    }
}

use parser::all;

pub fn day19_part1(input: &str) -> String {
    let (_, (workflows, parts)) = all(input).expect("parsing should succeed");

    // we need a map of workflows so we can address them by their name
    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name, workflow))
        .collect::<HashMap<&str, Workflow>>();

    // filter to accepted parts
    parts
        .iter()
        .filter_map(|part| {
            let mut workflow_name = "in";
            while workflow_name != "A" && workflow_name != "R" {
                let workflow = workflows
                    .get(workflow_name)
                    .expect("should always have the workflow");
                workflow_name = workflow.next(part)
            }
            match workflow_name {
                "A" => Some(part.x + part.m + part.a + part.s),
                "R" => None,
                _ => unreachable!("unpossible!"),
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day19_part2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::parser::*;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
", 2, vec!["px", "pv"], vec![3, 2])]
    fn test_day19_workflow_parser(
        #[case] input: &str,
        #[case] workflow_count: usize,
        #[case] names: Vec<&str>,
        #[case] rule_counts: Vec<usize>,
    ) {
        let (_, actual) = workflows(input).expect("parsing should succeed");
        assert_eq!(actual.len(), workflow_count);
        actual
            .iter()
            .zip(names.iter().zip(rule_counts.iter()))
            .for_each(|(workflow, (name, rule_count))| {
                assert_eq!(workflow.name, *name);
                assert_eq!(workflow.rules.len(), *rule_count);
            })
    }

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}

{x=787,m=2655,a=1222,s=2876}
{x=2127,m=1623,a=2188,s=1013}",
        2
    )]
    fn test_day19_all_parser(#[case] input: &str, #[case] expected: usize) {
        let (remainder, (workflows, parts)) = all(input).expect("parsing should succeed");
        assert_eq!(remainder.len(), 0);
        assert_eq!(workflows.len(), expected);
        assert_eq!(parts.len(), expected);
    }

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        "19114"
    )]
    fn test_day19_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day19_part1(input), expected);
    }

    #[test]
    #[ignore]
    fn test_day19_part2() {}
}
