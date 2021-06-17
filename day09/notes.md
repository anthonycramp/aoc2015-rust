# Advent of Code 2015 Day 09

[Advent of Code 2015 Day 09](https://adventofcode.com/2015/day/9)

## Thoughts

- Use regex to parse the input
- Compile a set of unique place names
- Maintain input data as a HashMap with keys (String, String) and values u32
- Each entry in the input, X to Y = z, results in two entries (x,y) -> z and
  (y,x) -> z
- Generate all permutations of the place names
- Walk each permutation two at a time, looking up distances, and aumming the
  result
