use std::collections::HashMap;

use itertools::Itertools;
use ModuleType::*;
use Signal::*;

use self::parser::parse;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Signal {
    High,
    Low,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool), // on/off
    Conjunction(HashMap<String, Signal>),
    Broadcast,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    r#type: ModuleType,
    destinations: Vec<&'a str>,
}

impl Module<'_> {
    fn send(&mut self, signal: Signal) -> Vec<(String, String, Signal)> {
        self.destinations
            .iter()
            .map(|module| (self.name.to_string(), module.to_string(), signal))
            .collect_vec()
    }

    fn receive(&mut self, from: String, signal: Signal) -> Vec<(String, String, Signal)> {
        match &self.r#type {
            FlipFlop(state) => match signal {
                High => vec![], // NOP
                Low => {
                    let old_state = state.clone(); // captured to prevent "borrow later used"
                    let new_state = !state;
                    self.r#type = FlipFlop(new_state);
                    match old_state {
                        true => self.send(Low),
                        false => self.send(High),
                    }
                }
            },
            Conjunction(state) => {
                // Copy the whole state :grimacing:
                // Otherwise I end up fighting with lifetimes and I'm not sure what to do.
                // A HashMap as an associated value for a enum was probably silly :)
                let mut new_state = state
                    .iter()
                    .map(|(k, v)| (k.to_owned(), *v))
                    .collect::<HashMap<String, Signal>>();

                new_state.insert(from.to_string(), signal);
                let all_high = new_state.iter().all(|(_, v)| match v {
                    High => true,
                    Low => false,
                });
                self.r#type = Conjunction(new_state);

                if all_high {
                    self.send(Low)
                } else {
                    self.send(High)
                }
            }
            Broadcast => self.send(signal),
        }
    }
}

pub fn day20_part1(input: &str) -> String {
    let (_, mut modules) = parse(input).expect("parsing should succeed");

    let mut counts = (0, 0);

    for _ in 1..=1000 {
        let broadcaster = modules
            .get_mut("broadcaster")
            .expect("needs to have a broadcaster");

        counts = (counts.0, counts.1 + 1);
        let mut signals = broadcaster.receive("in".to_string(), Low);
        while signals.len() > 0 {
            let new_signals = signals
                .iter()
                .filter_map(|signal| {
                    let receiver = modules.get_mut(&signal.1);
                    match signal.2 {
                        High => counts = (counts.0 + 1, counts.1),
                        Low => counts = (counts.0, counts.1 + 1),
                    }
                    match receiver {
                        Some(receiver) => Some(receiver.receive(signal.0.clone(), signal.2)),
                        _ => None,
                    }
                })
                .flatten()
                .collect_vec();
            signals = new_signals;
        }
    }

    (counts.0 * counts.1).to_string()
}

pub fn day20_part2(input: &str) -> String {
    let (_, mut modules) = parse(input).expect("parsing should succeed");

    let mut inputs: HashMap<String, (u32, u32)> = HashMap::new();

    // dbg!(inputs);

    // now monitor for pattern for when each input sends a high signal

    for button_presses in 1..=100000 {
        let broadcaster = modules
            .get_mut("broadcaster")
            .expect("needs to have a broadcaster");

        let mut signals = broadcaster.receive("in".to_string(), Low);
        while signals.len() > 0 {
            let new_signals = signals
                .iter()
                .filter_map(|signal| {
                    if signal.1 == "mg" {
                        match signal.2 {
                            High => {
                                if inputs.get(&signal.0).is_some() {
                                    let last = inputs.get(&signal.0).unwrap();
                                    let diff = button_presses - last.0;
                                    inputs.insert(signal.0.clone(), (button_presses, diff));
                                } else {
                                    inputs
                                        .insert(signal.0.clone(), (button_presses, button_presses));
                                }
                            }
                            _ => (),
                        };
                    }

                    let receiver = modules.get_mut(&signal.1);
                    match receiver {
                        Some(receiver) => Some(receiver.receive(signal.0.clone(), signal.2)),
                        _ => None,
                    }
                })
                .flatten()
                .collect_vec();
            signals = new_signals;
        }
    }

    dbg!(inputs);
    "put ^ in to an LCM calculator :)".to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[ignore]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        "32000000"
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        "11687500"
    )]
    fn test_day20_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day20_part1(input), expected);
    }

    #[rstest]
    #[case(
        Module{
            name: "broadcaster",
            r#type: Broadcast,
            destinations: vec!["a", "b"]
        },
    )]
    fn test_day20_broadcaster(#[case] mut module: Module) {
        assert_eq!(
            vec![
                ("broadcaster".to_string(), "a".to_string(), Low),
                ("broadcaster".to_string(), "b".to_string(), Low)
            ],
            module.receive("z".to_string(), Low)
        );
        assert_eq!(
            vec![
                ("broadcaster".to_string(), "a".to_string(), High),
                ("broadcaster".to_string(), "b".to_string(), High)
            ],
            module.receive("z".to_string(), High)
        );
        assert_eq!(
            vec![
                ("broadcaster".to_string(), "a".to_string(), High),
                ("broadcaster".to_string(), "b".to_string(), High)
            ],
            module.receive("z".to_string(), High)
        );
    }

    #[rstest]
    #[case(
        Module{
            name: "inv",
                r#type: Conjunction(
                    vec![("a".to_string(), Low), ("b".to_string(), Low)].into_iter().collect::<HashMap<String, Signal>>()
                ),
            destinations: vec!["a"],
        }
    )]
    fn test_day20_conjuction(#[case] mut module: Module) {
        assert_eq!(
            vec![("inv".to_string(), "a".to_string(), High)],
            module.receive("a".to_string(), High)
        ); // should send a high pulse until "b" flips to high
        assert_eq!(
            vec![("inv".to_string(), "a".to_string(), High)],
            module.receive("a".to_string(), High)
        );
        assert_eq!(
            vec![("inv".to_string(), "a".to_string(), High)],
            module.receive("b".to_string(), Low)
        );
        assert_eq!(
            vec![("inv".to_string(), "a".to_string(), Low)],
            module.receive("b".to_string(), High)
        );
        assert_eq!(
            vec![("inv".to_string(), "a".to_string(), High)],
            module.receive("a".to_string(), Low)
        );
    }

    #[rstest]
    #[case(
        Module{
            name: "f",
                r#type: FlipFlop(false),
            destinations: vec!["a"],
        }
    )]
    fn test_day20_flip_flops(#[case] mut module: Module) {
        assert_eq!(0, module.receive("a".to_string(), High).len());
        assert_eq!(
            vec![("f".to_string(), "a".to_string(), High)],
            module.receive("a".to_string(), Low)
        );
        assert_eq!(0, module.receive("a".to_string(), High).len());
        assert_eq!(
            vec![("f".to_string(), "a".to_string(), Low)],
            module.receive("a".to_string(), Low)
        );
        assert_eq!(
            vec![("f".to_string(), "a".to_string(), High)],
            module.receive("a".to_string(), Low)
        );
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, newline},
        combinator::opt,
        multi::separated_list1,
        sequence::{pair, separated_pair},
        IResult,
    };

    use super::*;

    // broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a
    pub fn parse(input: &str) -> IResult<&str, HashMap<String, Module>> {
        // build a hash map of all the modules.
        let (input, modules) = separated_list1(newline, module)(input)?;
        assert_eq!(input.len(), 0);
        let mut modules = modules
            .into_iter()
            .map(|module| (module.name.to_string(), module))
            .collect::<HashMap<String, Module>>();

        // build a hashmap of modules that act as inputs to conjunctions.
        // the key here is the destination (conjuctions). The value is the vec of module names
        // that act as inputs to that key.
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        modules.iter().for_each(|(_, module)| {
            module.destinations.iter().for_each(|destination_name| {
                let destination_name = destination_name.to_string();
                let _ = modules
                    .get(&destination_name)
                    .is_some_and(|destination_module| match &destination_module.r#type {
                        Conjunction(_) => {
                            if inputs.get(&destination_name).is_none() {
                                inputs.insert(destination_name, vec![module.name.to_string()]);
                            } else {
                                let i = inputs
                                    .get_mut(&destination_name)
                                    .expect("we tested it above?!");
                                i.push(module.name.to_string());
                            }
                            true
                        }
                        _ => false,
                    });
            });
        });

        // Now recreate the conjunction types
        modules
            .iter_mut()
            .for_each(|(_, module)| match &module.r#type {
                Conjunction(_) => {
                    let inputs = inputs.get(module.name).expect("Should always have inputs?");
                    let inputs = inputs
                        .iter()
                        .map(|input_name| (input_name.clone(), Low))
                        .collect::<HashMap<String, Signal>>();
                    module.r#type = Conjunction(inputs);
                }
                _ => (),
            });

        Ok((input, modules))
    }

    fn module(input: &str) -> IResult<&str, Module> {
        let (input, ((r#type, name), destinations)) = separated_pair(
            pair(opt(alt((tag("%"), tag("&")))), alpha1),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        )(input)?;
        let module = match r#type {
            None => {
                assert!(name == "broadcaster");
                Module {
                    name,
                    r#type: Broadcast,
                    destinations,
                }
            }
            Some("&") => Module {
                name,
                r#type: Conjunction(HashMap::new()),
                destinations,
            },
            Some("%") => Module {
                name,
                r#type: FlipFlop(false),
                destinations,
            },
            _ => unreachable!("unpossible!"),
        };

        Ok((input, module))
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
        )]
        fn test_day20_parser(#[case] input: &str) {
            let (remainder, modules) = parse(input).expect("parsing should succeed");
            assert_eq!(remainder.len(), 0);
            assert_eq!(modules.len(), 5);

            assert!(modules
                .get("broadcaster")
                .is_some_and(|module| module.destinations.len() == 3));

            assert!(modules
                .get("b")
                .is_some_and(|module| module.destinations.len() == 1
                    && match module.r#type {
                        FlipFlop(_) => true,
                        _ => false,
                    }));

            assert!(modules
                .get("inv")
                .is_some_and(|module| module.destinations.len() == 1
                    && match &module.r#type {
                        Conjunction(inputs) => inputs.len() == 1,
                        _ => false,
                    }));

            let inverter = modules.get("inv").unwrap();
            match &inverter.r#type {
                Conjunction(inputs) => match inputs.get("c") {
                    Some(Low) => (),
                    Some(High) => unreachable!("should have started low!"),
                    _ => unreachable!("Should have had a c"),
                },
                _ => unreachable!(),
            }
        }
    }
}
