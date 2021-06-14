use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const INPUT: &'static str = include_str!("day07.txt");

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Clone)]
enum LogicGate {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Constant,
}

#[derive(Debug, PartialEq, Clone)]
enum Input {
    Wire(String),
    Signal(u16),
}

#[derive(Debug, Clone)]
struct Gate {
    output_wire: String,
    gate_type: LogicGate,
    input1: Input,
    input2: Option<Input>,
}

impl From<&'static str> for Gate {
    fn from(input: &'static str) -> Self {
        lazy_static! {
            static ref TOP_RE: Regex = Regex::new(r"(.*) -> (.*)").unwrap();
            static ref LHS_BINARY_RE: Regex = Regex::new(r"(.*) (.*) (.*)").unwrap();
            static ref LHS_UNARY_RE: Regex = Regex::new(r"(.*) (.*)").unwrap();
        }

        let fields = TOP_RE.captures(input).unwrap();

        if let Some(lhs) = LHS_BINARY_RE.captures(fields.get(1).unwrap().as_str()) {
            let output_wire = String::from(fields.get(2).unwrap().as_str());
            let op1 = String::from(lhs.get(1).unwrap().as_str());
            let op2 = String::from(lhs.get(3).unwrap().as_str());
            let gate_type_str = lhs.get(2).unwrap().as_str();
            let gate_type = match gate_type_str {
                "AND" => LogicGate::And,
                "OR" => LogicGate::Or,
                "LSHIFT" => LogicGate::Lshift,
                "RSHIFT" => LogicGate::Rshift,
                _ => panic!("Unknown gate specified: {}", gate_type_str),
            };

            let input1 = if let Ok(v) = op1.parse::<u16>() {
                Input::Signal(v)
            } else {
                Input::Wire(op1)
            };

            let input2 = Some(if let Ok(v) = op2.parse::<u16>() {
                Input::Signal(v)
            } else {
                Input::Wire(op2)
            });

            Self {
                output_wire,
                gate_type,
                input1,
                input2,
            }
        } else if let Some(lhs) = LHS_UNARY_RE.captures(fields.get(1).unwrap().as_str()) {
            let op1 = String::from(lhs.get(2).unwrap().as_str());
            let input1 = if let Ok(v) = op1.parse::<u16>() {
                Input::Signal(v)
            } else {
                Input::Wire(op1)
            };

            Self {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Not,
                input1: input1,
                input2: None,
            }
        } else {
            Self {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Constant,
                input1: Input::Signal(
                    String::from(fields.get(1).unwrap().as_str())
                        .parse()
                        .unwrap(),
                ),
                input2: None,
            }
        }
    }
}

struct Circuit {
    gates_by_output_wire: HashMap<String, Gate>,
}

impl From<&'static str> for Circuit {
    fn from(input: &'static str) -> Self {
        let mut gates_by_output_wire = HashMap::new();

        for line in input.lines() {
            let gate = Gate::from(line);
            gates_by_output_wire.insert(gate.output_wire.clone(), gate.clone());
        }

        Self {
            gates_by_output_wire,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary_gate() {
        let input = "x AND y -> d";
        let parsed_input = Gate::from(input);

        assert_eq!(parsed_input.output_wire, String::from("d"));
        assert_eq!(parsed_input.gate_type, LogicGate::And);
        assert_eq!(parsed_input.input1, Input::Wire(String::from("x")));
        assert_eq!(parsed_input.input2, Some(Input::Wire(String::from("y"))));
    }

    #[test]
    fn test_parse_unary_gate() {
        let input = "NOT x -> h";
        let parsed_input = Gate::from(input);

        assert_eq!(parsed_input.output_wire, String::from("h"));
        assert_eq!(parsed_input.gate_type, LogicGate::Not);
        assert_eq!(parsed_input.input1, Input::Wire(String::from("x")));
        assert_eq!(parsed_input.input2, None);
    }

    #[test]
    fn test_parse_constant() {
        let input = "123 -> x";
        let parsed_input = Gate::from(input);

        assert_eq!(parsed_input.output_wire, String::from("x"));
        assert_eq!(parsed_input.gate_type, LogicGate::Constant);
        assert_eq!(parsed_input.input1, Input::Signal(123));
        assert_eq!(parsed_input.input2, None);
    }

    #[test]
    fn test_parse_lshift() {
        let input = "x LSHIFT 2 -> f";
        let parsed_input = Gate::from(input);

        assert_eq!(parsed_input.output_wire, String::from("f"));
        assert_eq!(parsed_input.gate_type, LogicGate::Lshift);
        assert_eq!(parsed_input.input1, Input::Wire(String::from("x")));
        assert_eq!(parsed_input.input2, Some(Input::Signal(2)));
    }

    #[test]
    fn test_part1() {
        let input = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        let circuit = Circuit::from(input);
        assert_eq!(circuit.gates_by_output_wire.len(), 8);
        // circuit.reduce();

        // assert_eq!(circuit.wire("d").unwrap(), 72);
        // assert_eq!(circuit.wire("e").unwrap(), 507);
        // assert_eq!(circuit.wire("f").unwrap(), 492);
        // assert_eq!(circuit.wire("g").unwrap(), 114);
        // assert_eq!(circuit.wire("h").unwrap(), 65412);
        // assert_eq!(circuit.wire("i").unwrap(), 65079);
        // assert_eq!(circuit.wire("x").unwrap(), 123);
        // assert_eq!(circuit.wire("y").unwrap(), 456);
    }
}
