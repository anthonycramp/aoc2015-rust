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

## Reflection

Went largely as I'd planned above. Still relying on the compiler to tell me when
something is being used after borrow. My solution in most of these cases is to
just tack `.clone()` everywhere.

Need to think about using `.cloned()` on an iterator to go from `Item=&T` to
`Item=T`. In the case, I can remove `.cloned()` by changing the first parameter
of `compute_distance` from `&[String]` to `&[&String]`.
