use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Copy, Clone)]
enum XmasProperty {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
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

    fn to_range(&self, passing: bool) -> RangeInclusive<u32> {
        match self {
            GreaterThan(_, value) => {
                if passing {
                    value + 1..=4000
                } else {
                    1..=*value
                }
            }
            LessThan(_, value) => {
                if passing {
                    1..=(value - 1)
                } else {
                    *value..=4000
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
use ranges::Ranges;

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

pub fn day19_part2(input: &str) -> String {
    // each part component can be 1-4000
    // there are 4 components to each part.
    // there are 2.56e14 combinations of parts?
    // the sample answer is 1.67...e14

    // will a repeating pattern emerge? probably not.

    // the workflows use ranges.
    // walk each workflow and determine the ranges that lead to success?
    let (_, (workflows, _)) = all(input).expect("should parse");
    // we need a map of workflows so we can address them by their name
    let workflows = workflows
        .into_iter()
        .map(|workflow| (workflow.name, workflow))
        .collect::<HashMap<&str, Workflow>>();

    // This is path finding? recursion?
    let mut paths = AcceptablePaths {
        current_path: vec![],
        acceptable_paths: vec![],
        workflows: &workflows,
    };
    paths.acceptable_paths("in");

    // paths.acceptable_paths can now be iterated to build up ranges
    // the product of the valid ranges for each XMAS in each path then summed is the answer.
    paths
        .acceptable_paths
        .iter()
        .map(|path| {
            //
            let mut acceptable_x = Ranges::from(1..=4000);
            let mut acceptable_m = Ranges::from(1..=4000);
            let mut acceptable_a = Ranges::from(1..=4000);
            let mut acceptable_s = Ranges::from(1..=4000);

            path.iter().for_each(|(rule, required)| match rule.test {
                None => (),
                Some(test) => {
                    let property = match test {
                        GreaterThan(p, _) => p,
                        LessThan(p, _) => p,
                    };
                    let range = test.to_range(*required);
                    match property {
                        X => {
                            let r = Ranges::from(range);
                            acceptable_x &= r;
                        }
                        M => {
                            acceptable_m &= Ranges::from(range);
                        }
                        A => {
                            acceptable_a &= Ranges::from(range);
                        }
                        S => {
                            acceptable_s &= Ranges::from(range);
                        }
                    };
                }
            });
            let mut x_cnt = acceptable_x
                .as_slice()
                .iter()
                .fold(0, |acc, range| acc + range.into_iter().count());
            let mut m_cnt = acceptable_m
                .as_slice()
                .iter()
                .fold(0, |acc, range| acc + range.into_iter().count());
            let mut a_cnt = acceptable_a
                .as_slice()
                .iter()
                .fold(0, |acc, range| acc + range.into_iter().count());
            let mut s_cnt = acceptable_s
                .as_slice()
                .iter()
                .fold(0, |acc, range| acc + range.into_iter().count());

            if x_cnt == 0 {
                x_cnt = 4000;
            }
            if m_cnt == 0 {
                m_cnt = 4000;
            }
            if a_cnt == 0 {
                a_cnt = 4000;
            }
            if s_cnt == 0 {
                s_cnt = 4000;
            }
            x_cnt as u64 * m_cnt as u64 * a_cnt as u64 * s_cnt as u64
        })
        .sum::<u64>()
        .to_string()
}

#[derive(Debug)]
struct AcceptablePaths<'a> {
    current_path: Vec<(Rule<'a>, bool)>,
    acceptable_paths: Vec<Vec<(Rule<'a>, bool)>>,
    workflows: &'a HashMap<&'a str, Workflow<'a>>,
}

impl AcceptablePaths<'_> {
    fn acceptable_paths(&'_ mut self, workflow: &'_ str) {
        if workflow == "A" {
            let last_path = self.current_path.clone();
            self.acceptable_paths.push(last_path);
            return;
        } else if workflow == "R" {
            return;
        } else {
            // otherwise, recurse in to each branch of the next rule
            let workflow = self.workflows.get(workflow).expect("unpossibruh");
            workflow.rules.iter().enumerate().for_each(|(_, rule)| {
                self.current_path.push((*rule, true));
                self.acceptable_paths(rule.destination_workflow);
                self.current_path.pop();

                // push the false branch now and go to the next one
                self.current_path.push((*rule, false));
            });

            // Pop each false that we pushed
            for _ in 0..workflow.rules.len() {
                self.current_path.pop();
            }
            return;
        }
    }
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
        "167409079868000"
    )]
    fn test_day19_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day19_part2(input), expected);
    }
}
