# Advent of Code 2015 Day 19

[Advent of Code 2015 Day 19](https://adventofcode.com/2015/day/19)

## Thoughts

- Use `HashMap<String, Vec<String>>` to store replacements
- Iterate all keys, value pairs and do replacements
- Store replacements in a HashSet to ensure uniqueness
- Return the length of the HashSet
