mod test {
    use algorithm::binary_search::binary_search;
    static ARRAY: [i32; 8] = [1, 2, 3, 5, 5, 5, 8, 9];

    #[test]
    fn case_1() {
        // 找到第一个">=5"的元素
        let ret = binary_search(&ARRAY, |s| *s < 5, false).unwrap();
        assert_eq!(ret, 3);
    }

    #[test]
    fn case_2() {
        // 找到最后一个"<5"的元素
        let ret = binary_search(&ARRAY, |s| *s < 5, true).unwrap();
        assert_eq!(ret, 2);
    }

    #[test]
    fn case_3() {
        // 找到第一个">5"的元素
        let ret = binary_search(&ARRAY, |s| *s <= 5, false).unwrap();
        assert_eq!(ret, 6);
    }

    #[test]
    fn case_4() {
        // 找到最后一个"<=5"的元素
        let ret = binary_search(&ARRAY, |s| *s <= 5, true).unwrap();
        assert_eq!(ret, 5);
    }

    #[test]
    fn case_5() {
        // 找到最后一个小于0的元素
        let ret = binary_search(&ARRAY, |s| *s < 0, true);
        assert_eq!(ret, None);
    }

    #[test]
    fn case_6() {
        // 找到第一个大于等于0的元素
        let ret = binary_search(&ARRAY, |s| *s < 0, false).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn case_7() {
        // 找到第一个大于等于0的元素
        let ret = binary_search(&ARRAY, |s| *s < 10, false);
        assert_eq!(ret, None);
    }

    #[test]
    fn case_8() {
        // 找到最后第一个小于10的元素
        let ret = binary_search(&ARRAY, |s| *s < 10, true).unwrap();
        assert_eq!(ret, ARRAY.len() - 1);
    }

    #[test]
    fn case_9() {
        // 找到第一个第一个大于等于于10的元素
        let ret = binary_search(&ARRAY, |s| *s < 10, false);
        assert_eq!(ret, None);
    }
    
    #[test]
    fn case_10() {
        let array = [4, 4, 4, 4, 6, 6, 6, 6, 6];
        let ret = binary_search(&array, |s| *s == 4, false).unwrap();
        assert_eq!(ret, 4);
    }
}