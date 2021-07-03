# Advent of Code 2015 Day 19

[Advent of Code 2015 Day 19](https://adventofcode.com/2015/day/19)

## Thoughts

- Use `HashMap<String, Vec<String>>` to store replacements
- Iterate all keys, value pairs and do replacements
- Store replacements in a HashSet to ensure uniqueness
- Return the length of the HashSet

## Reflection

- Brute force solution for Part 2 work for the small test cases but is too
  computationally expensive for the full problem. I stopped it running after
  about two hours, at which point it was consuming 16 GB.
- Read through some of the solutions
  [on Reddit](https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/).
  Looks like there is a simple equation to provide the solution to Part 2. Also,
  a greedy algorithm that reduces the target molecule back to e can return the
  correct answer. Knowledge of Context Free Grammars (CFGs) is helpful?
