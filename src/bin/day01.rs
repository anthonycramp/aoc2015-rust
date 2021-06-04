fn main() {}

fn follow_directions(directions: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        assert_eq!(follow_directions(""), 0);
    }
}
