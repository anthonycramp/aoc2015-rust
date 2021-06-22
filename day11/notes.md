# Advent of Code 2015 Day 11

[Advent of Code 2015 Day 11](https://adventofcode.com/2015/day/11)

## Thoughts

Two main things:

- Increment a password (base 26 arithmetic)
- Check a password for validity

## Reflection

- Experimented with a trait for defining Password functionality and added that
  trait to the String type. Works OK but I'm not convinced of the software
  design. Especially because there is nothing that actually takes Password as a
  type. I might need a PasswordPolicy type and move the validity checking there.
- Also got caught out by a substraction underflow on u8 types. I need to be more
  conscientous when using unsigned types. Although, Rust caught the problem with
  a runtime panic.
- There are some solutions in r/adventofcode that are incredibly terse and quite
  a few make extensive use of regular expressions. Some extra knowledge here
  might be useful, but only for readers also familiar with regexs.
- This particular problem can be solved relatively simply by inspection instead
  of coding a solution.
