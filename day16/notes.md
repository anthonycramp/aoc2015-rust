# Advent of Code 2015 Day 16

[Advent of Code 2015 Day 16](https://adventofcode.com/2015/day/16)

## Thoughts

- seems like a fairly simple parse and match problem
- Option to the rescue!

## Reflection

- Went pretty much as planned
- Kind of long winded to code, would be interesting to review other solutions
- Careful with regex's being greedy. Use ? to stop at first match, or be more
  specific in captures. E.g., specify character classes rather than a generic
  .\* match.
- Interesting suggestion from clippy. Change

```rust
let s = S::default();
s.a = 123;
```

to

```rust
let s = S {
  a: 123,
  ..Default::default()
};

```

Assume this requires implementation of `Default`.

- The above is
  [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax),
  and can be use other associated methdos (not just Default::default()).
