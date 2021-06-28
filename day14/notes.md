# Advent of Code 2015 Day 14

[Advent of Code 2015 Day 14](https://adventofcode.com/2015/day/14)

## Thoughts

- Simple time stepped simulation?
- Could be an event driven simulation to improve performance
- Suspect Part 2 will introduce some inter time checks that will benefit from a
  time stepped, rather then event stepped, approach
- Going to implement this myself for now but I'm sure there are Rust sim engines
  that could be leveraged
- Actually, Part 1 can be solved analytically. For a reindeer that can do speed
  S for t_s seconds and then rests for t_r seconds. This combination is the
  cycle time t_c = t_s + t_r. Over t seconds, it goes through A = t / t_c
  complete cycles and one partial cycle of duration B = t % t_c. Total distance
  covered is s \* A + s \* min(t_s, B). (although, I still think Part 2 will
  require an iterative solution)
