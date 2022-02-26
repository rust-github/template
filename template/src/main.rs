#![doc = include_str!("../README.md")]

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        assert_eq!(1 + 1, 2);
    }
}
