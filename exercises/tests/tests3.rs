// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` for hints :)


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        let num: i32 = 12;
        assert!(is_even(num), true);
    }

    #[test]
    fn is_false_when_odd() {
        let num: i32 = 5;
        assert!(!is_even(num), false);
    }
}
