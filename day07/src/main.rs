use std::collections::HashMap;

const INPUT: &'static str = include_str!("day07.txt");

fn main() {
    println!("Hello, world!");
}

#[derive(Hash, Eq, PartialEq)]
struct Wire {
    identifier: &'static str,
}

struct Value {
    value: u16,
}

enum Input {
    Wire(Wire),
    Value(Value),
}

struct Binary {
    op1: Input,
    op2: Input,
}

struct Unary {
    op: Input,
}

struct Constant {
    value: Value,
}

enum LogicGate {
    And(Binary),
    Or(Binary),
    Lshift(Binary),
    Rshift(Binary),
    Not(Unary),
    Signal(Constant),
}

impl LogicGate {
    fn reduce(&self, circuit: &Circuit) -> Option<LogicGate> {
        None
    }
}

struct Circuit {
    circuit: HashMap<Wire, LogicGate>,
}

impl From<&str> for Circuit {
    fn from(input: &str) -> Self {
        Circuit {
            circuit: HashMap::new(),
        }
    }
}

impl Circuit {
    fn wire(&self, identifier: &'static str) -> Option<u16> {
        match self.circuit.get(&Wire { identifier }) {
            Some(gate) => match gate {
                LogicGate::Signal(c) => Some(c.value.value),
                _ => None,
            },
            None => None,
        }
    }

    fn reduce(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
        circuit.reduce();

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
