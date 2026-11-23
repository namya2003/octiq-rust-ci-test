pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn multiplies() {
        assert_eq!(multiply(2, 3), 6);
    }
}
