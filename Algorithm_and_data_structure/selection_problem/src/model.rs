#[cfg(test)]
mod tests {
    use crate::algrithm::*;

    #[test]
    fn test_basic() {
        let mut arr = [3, 1, 4, 1, 5, 9];
        assert_eq!(*select(&mut arr, 3).unwrap(), 3);
    }

    #[test]
    fn test_order_out_of_bounds() {
        let mut arr = [1, 2, 3];
        assert!(select(&mut arr, 0).is_err());
        assert!(select(&mut arr, 4).is_err());
    }

    #[test]
    fn test_duplicates() {
        let mut arr = [2, 2, 2];
        assert_eq!(*select(&mut arr, 2).unwrap(), 2);
    }
}