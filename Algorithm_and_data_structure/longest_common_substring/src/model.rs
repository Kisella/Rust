#[cfg(test)]
mod tests {
    use crate::algrithm::*;

    #[test]
    fn test_basic() {
        let x = vec!["A", "B", "C", "D", "E"];
        let y = vec!["B", "C", "D"];
        assert_eq!(
            longest_common_substring(&x, &y),
            &["B", "C", "D"]
        );
    }

    #[test]
    fn test_no_common() {
        let x = vec![1, 2, 3];
        let y = vec![4, 5, 6];
        assert!(longest_common_substring(&x, &y).is_empty());
    }

    #[test]
    fn test_empty_input() {
        assert!(longest_common_substring::<i32>(&[], &[1]).is_empty());
    }
}