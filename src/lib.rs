// Author; Mitch Allen

/// Add two numbers together
///
/// This function takes two numbers and adds them together
/// and returns the result.
///
/// ```
/// let result = rust_lib_01::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Subtract one number from another
///
/// This function subtracts one number from the other
/// and returns the result.
///
/// ```
/// let result = rust_lib_01::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_sub() {
        let result = sub(6, 5);
        assert_eq!(result, 1);
    }
}
