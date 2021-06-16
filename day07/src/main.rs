use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const INPUT: &'static str = include_str!("day07.txt");

fn main() {
    println!("Day 07 Part 1: {}", part1(INPUT));
    println!("Day 07 Part 2: {}", part2(INPUT));
}

fn part1(input: &'static str) -> u16 {
    let mut circuit = Circuit::from(input);
    circuit.solve();
    circuit.wire("a").unwrap()
}

fn part2(input: &'static str) -> u16 {
    let part1_output = part1(input);
    let mut circuit = Circuit::from(input);
    circuit.set_wire("b", part1_output);
    circuit.solve();
    circuit.wire("a").unwrap()
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

impl Input {
    fn is_signal(&self) -> bool {
        match self {
            Input::Wire(_) => false,
            Input::Signal(_) => true,
        }
    }
}

impl From<&String> for Input {
    fn from(input: &String) -> Self {
        if let Ok(v) = input.parse::<u16>() {
            Input::Signal(v)
        } else {
            Input::Wire(input.clone())
        }
    }
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

            let input1 = Input::from(&op1);
            let input2 = Some(Input::from(&op2));

            Self {
                output_wire,
                gate_type,
                input1,
                input2,
            }
        } else if let Some(lhs) = LHS_UNARY_RE.captures(fields.get(1).unwrap().as_str()) {
            let op1 = String::from(lhs.get(2).unwrap().as_str());
            let input1 = Input::from(&op1);

            Self {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Not,
                input1: input1,
                input2: None,
            }
        } else {
            let op1 = String::from(fields.get(1).unwrap().as_str());
            let input1 = Input::from(&op1);
            Self {
                output_wire: String::from(fields.get(2).unwrap().as_str()),
                gate_type: LogicGate::Constant,
                input1: input1,
                input2: None,
            }
        }
    }
}

#[derive(Clone)]
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

impl Circuit {
    /// Return the value of the signal on the specified wire if it has
    /// already been computed. Otherwise return None.
    fn wire(&self, wire_id: &str) -> Option<u16> {
        if let Some(gate) = self.gates_by_output_wire.get(&String::from(wire_id)) {
            match gate.gate_type {
                LogicGate::Constant => match gate.input1 {
                    Input::Signal(v) => Some(v),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }

    fn set_wire(&mut self, wire_id: &str, signal: u16) {
        self.gates_by_output_wire.insert(
            String::from(wire_id),
            Gate {
                output_wire: String::from(wire_id),
                gate_type: LogicGate::Constant,
                input1: Input::Signal(signal),
                input2: None,
            },
        );
    }

    fn solve(&mut self) {
        // infinite loop until the circuit doesn't change any further
        loop {
            let mut circuit_changed = false;

            // iterate through the circuit
            // try to resolve any non-Signal logic gates by
            // looking up the input wires in the circuit
            // we'll use a copy of the circuit to look up wires
            let circuit_copy = self.clone();

            for (_, value) in self.gates_by_output_wire.iter_mut() {
                // value will have one or two inputs, each of which may be
                // a wire or a signal. If it is a wire, we want to look it
                // up to see if it resolves to a signal. Ultimately, we want
                // one or two signals to proceed to application of gate function
                // let's skip if value is a signal
                if LogicGate::Constant == value.gate_type && value.input1.is_signal() {
                    continue;
                }

                let input1 = match &value.input1 {
                    Input::Wire(w) => circuit_copy.wire(&w),
                    Input::Signal(v) => Some(*v),
                };

                let input2 = match &value.input2 {
                    Some(input) => match input {
                        Input::Wire(w) => circuit_copy.wire(&w),
                        Input::Signal(v) => Some(*v),
                    },
                    _ => None,
                };

                match (input1, input2) {
                    (Some(v1), Some(v2)) => {
                        circuit_changed = true;
                        match &value.gate_type {
                            LogicGate::And => (*value).input1 = Input::Signal(v1 & v2),
                            LogicGate::Or => (*value).input1 = Input::Signal(v1 | v2),
                            LogicGate::Lshift => (*value).input1 = Input::Signal(v1 << v2),
                            LogicGate::Rshift => (*value).input1 = Input::Signal(v1 >> v2),
                            _ => panic!("Expected a binary logic gate: {:?}", value),
                        }
                        (*value).input2 = None;
                        (*value).gate_type = LogicGate::Constant;
                    }
                    (Some(v1), None) => match &value.gate_type {
                        LogicGate::Not => {
                            circuit_changed = true;
                            (*value).input1 = Input::Signal(!v1);
                            (*value).gate_type = LogicGate::Constant;
                        }
                        LogicGate::Constant => {
                            circuit_changed = true;
                            (*value).input1 = Input::Signal(v1);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            if !circuit_changed {
                break;
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
    fn test_parse_rshift() {
        let input = "y RSHIFT 2 -> g";
        let parsed_input = Gate::from(input);

        assert_eq!(parsed_input.output_wire, String::from("g"));
        assert_eq!(parsed_input.gate_type, LogicGate::Rshift);
        assert_eq!(parsed_input.input1, Input::Wire(String::from("y")));
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

        let mut circuit = Circuit::from(input);
        assert_eq!(circuit.gates_by_output_wire.len(), 8);
        circuit.solve();

        assert_eq!(circuit.wire("d").unwrap(), 72);
        assert_eq!(circuit.wire("e").unwrap(), 507);
        assert_eq!(circuit.wire("f").unwrap(), 492);
        assert_eq!(circuit.wire("g").unwrap(), 114);
        assert_eq!(circuit.wire("h").unwrap(), 65412);
        assert_eq!(circuit.wire("i").unwrap(), 65079);
        assert_eq!(circuit.wire("x").unwrap(), 123);
        assert_eq!(circuit.wire("y").unwrap(), 456);
    }
}
