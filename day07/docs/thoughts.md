# Day 07

[Day 07](https://adventofcode.com/2015/day/7)

## Thoughts

Seems to be a rule rewriting problem. Can I maintain a map from wire identifier
to value or expression and then reduce the map until no further reduction
occurs?

Parsing input will be a little more complex because the expressions are not
consistent for all operations.

What things do we have? Values (u16), Identifiers (&str), Logic Gates (one or
two Inputs, that can be Values or Identifiers, and one Output that is an
Identifier).

How does Rust represent Union types? enums are basically what I'm after.

```rust
enum Input {
    Value(val),
    Wire(&'static str),
}
```

There are three general types of Logic Gate, Binary, Unary, and Constant. There
is no output captured here. Instead, the output wire is captured in the
overarching Circuit map.

```rust
struct Binary {
    op1: Input,
    op2: Input,
}

struct Unary {
    op: Input,
}

struct Constant {
    in: Value,
}
```

Then, a Logic Gate is an enum:

```rust
enum LogicGate {
    Signal(Constant),
    And(Binary),
    Or(Binary),
    Lshift(Binary),
    Rshift(Binary),
    Not(Unary),
}
```

A Logic Gate can be reduced if both its inputs are Values. At this point, the
Logic Gate can be replaced by a Constant, which can automatically be replaced by
its contained Value.

The Circuit is a map from Wire to the inbound Logic Gate:
`HashMap<Wire, LogicGate>`.

```rust
loop {
    let mut circuit_changed = false;
    for k, v in circuit {
        if let Some(reduction) = v.reduce(&circuit) {
            circuit.insert(k, reduction);
            circuit_changed = true;
        }
    }

    if !circuit_changed {
        break;
    }
}
```

## 2021-06-14

I began implementing the above but am not completely comfortable with it. This
is partly due to my Rust familiarity. I have an idea of how I'd tackle this in
C++, but how to handle mutable HashMap entries in Rust is still very stochastic
for me.

I'm going to step back and take it one step at a time instead of trying to do an
up front desing. I'll start with parsing the input.

There will be one top level regex to split the left and right of the ->. Then,
two other regex's will parse the left hand side.

Parse into a generic struct to start with.
