use regex::Regex;

#[macro_use]
extern crate lazy_static;

const INPUT: &'static str = include_str!("day07.txt");

fn main() {
    println!("Hello, world!");
}

struct ParsedInput {
    output_wire: &'static str,
    gate_type: Option<&'static str>,
    op1: &'static str,
    op2: Option<&'static str>,
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
            ParsedInput {
                output_wire: fields.get(2).unwrap().as_str(),
                gate_type: Some(lhs.get(2).unwrap().as_str()),
                op1: lhs.get(1).unwrap().as_str(),
                op2: Some(lhs.get(3).unwrap().as_str()),
            }
        } else if let Some(lhs) = LHS_UNARY_RE.captures(fields.get(1).unwrap().as_str()) {
            ParsedInput {
                output_wire: fields.get(2).unwrap().as_str(),
                gate_type: Some(lhs.get(1).unwrap().as_str()),
                op1: lhs.get(2).unwrap().as_str(),
                op2: None,
            }
        } else {
            ParsedInput {
                output_wire: fields.get(2).unwrap().as_str(),
                gate_type: None,
                op1: fields.get(1).unwrap().as_str(),
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

        assert_eq!(parsed_input.output_wire, "d");
        assert_eq!(parsed_input.gate_type, Some("AND"));
        assert_eq!(parsed_input.op1, "x");
        assert_eq!(parsed_input.op2, Some("y"));
    }

    #[test]
    fn test_parse_unary_gate() {
        let input = "NOT x -> h";
        let parsed_input = ParsedInput::from(input);

        assert_eq!(parsed_input.output_wire, "h");
        assert_eq!(parsed_input.gate_type, Some("NOT"));
        assert_eq!(parsed_input.op1, "x");
        assert_eq!(parsed_input.op2, None);
    }

    #[test]
    fn test_parse_constant() {
        let input = "123 -> x";
        let parsed_input = ParsedInput::from(input);

        assert_eq!(parsed_input.output_wire, "x");
        assert_eq!(parsed_input.gate_type, None);
        assert_eq!(parsed_input.op1, "123");
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
