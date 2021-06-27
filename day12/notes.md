# Advent of Code 2015 Day 12

[Advent of Code 2015 Day 12](https://adventofcode.com/2015/day/12)

## Thoughts

Part 1

- split input on json punctionation characters
- attemt to parse the split fields as i32
- sum the fields that parse as i32

Part 2

- parse JSON into structure
- prune the structure as appropriate

## Reflection

- `str.split` can take a predicate. The `char` type has a bunch of predicates
  for checking character classes.
- The `serde_json` crate (used in the solution for Part 2) is really useful
