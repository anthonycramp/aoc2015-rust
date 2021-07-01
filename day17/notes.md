# Advent of Code 2015 Day 17

[Advent of Code 2015 Day 17](https://adventofcode.com/2015/day/17)

## Thoughts

- Brute force solution is to create all combinations of length 1, 2, ..., n and
  check if they sum to the target
- Should also be amenable to a dynamic programming solution

## Reflection

- Turns out brute force was quick enough
- all thanks to the `itertools` crate and the combinations() method
