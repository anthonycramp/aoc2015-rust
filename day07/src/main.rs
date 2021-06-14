use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &'static str = include_str!("day07.txt");

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum LogicGate {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    Constant,
}

struct ParsedInput {
    output_wire: String,
    gate_type: LogicGate,
    op1: String,
    op2: Option<String>,
}

impl From<&'static str> for ParsedInput {
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
            let op2 = Some(String::from(lhs.get(3).unwrap().as_str()));
            let gate_type_str = lhs.get(2).unwrap().as_str();
            let gate_type = match gate_type_str {
                "AND" => LogicGate::And,
                "OR" => LogicGate::Or,
                "LSHIFT" => LogicGate::Lshift,
                "RSHIFT" => LogicGate::Rshift,
                _ => panic!("Unknown gate specified: {}", gate_type_str),
            };

            ParsedInput {
                output_wire,
                gate_type,
                op1,
                op2,
            }
        } else if let Some(lhs) = LHS_UNARY_RE.captures(fields.get(1).unwrap().as_str()) {
            ParsedInput {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Not,
                op1: String::from(lhs.get(2).unwrap().as_str()),
                op2: None,
            }
        } else {
            ParsedInput {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Constant,
                op1: String::from(fields.get(1).unwrap().as_str()),
                op2: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary_gate() {
        let input = "x AND y -> d";
        let parsed_input = ParsedInput::from(input);

        assert_eq!(parsed_input.output_wire, String::from("d"));
        assert_eq!(parsed_input.gate_type, LogicGate::And);
        assert_eq!(parsed_input.op1, String::from("x"));
        assert_eq!(parsed_input.op2, Some(String::from("y")));
    }

    #[test]
    fn test_parse_unary_gate() {
        let input = "NOT x -> h";
        let parsed_input = ParsedInput::from(input);

        assert_eq!(parsed_input.output_wire, String::from("h"));
        assert_eq!(parsed_input.gate_type, LogicGate::Not);
        assert_eq!(parsed_input.op1, String::from("x"));
        assert_eq!(parsed_input.op2, None);
    }

    #[test]
    fn test_parse_constant() {
        let input = "123 -> x";
        let parsed_input = ParsedInput::from(input);

        assert_eq!(parsed_input.output_wire, String::from("x"));
        assert_eq!(parsed_input.gate_type, LogicGate::Constant);
        assert_eq!(parsed_input.op1, String::from("123"));
        assert_eq!(parsed_input.op2, None);
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

        // let mut circuit = Circuit::from(input);
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
