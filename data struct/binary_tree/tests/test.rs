mod case1 {
    use std::collections::VecDeque;

    use binary_tree::models::*;
    use tree_node::*;

    #[test]
    fn case_1() {
        let values = [
            Some('B'),
            Some('D'),
            None,
            None,
            Some('C'),
            Some('E'),
            None,
            None,
            Some('F'),
            None,
            None,
        ];
        let tree = BinaryTree::pre_order_creat(VecDeque::from(values));
        assert_eq!(tree.level_order_traversal(), ['B', 'D', 'C', 'E', 'F']);
        assert_eq!(tree.pre_order_traversal(), ['B', 'D', 'C', 'E', 'F']);
        assert_eq!(tree.mid_order_traversal(), ['D', 'B', 'E', 'C', 'F']);
        assert_eq!(tree.post_order_traversal(), ['D', 'E', 'F', 'C', 'B']);
    }

    #[test]
    fn case_2() {
        let values = [
            Some(1),
            Some(2),
            Some(4),
            None,
            None,
            Some(5),
            None,
            None,
            Some(3),
            Some(6),
            None,
            None,
            Some(7),
            None,
            None,
        ];
        let tree = BinaryTree::pre_order_creat(VecDeque::from(values));
        assert_eq!(tree.level_order_traversal(), [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(tree.pre_order_traversal(), [1, 2, 4, 5, 3, 6, 7]);
        assert_eq!(tree.mid_order_traversal(), [4, 2, 5, 1, 6, 3, 7]);
        assert_eq!(tree.post_order_traversal(), [4, 5, 2, 6, 7, 3, 1]);
    }
    #[test]
    fn case_3() {
        let values = [
            Some(1),
            Some(2),
            None,
            None,
            Some(3),
            Some(6),
            None,
            None,
            None,
        ];
        let tree = BinaryTree::pre_order_creat(VecDeque::from(values));
        assert_eq!(tree.level_order_traversal(), [1, 2, 3, 6]);
        assert_eq!(tree.pre_order_traversal(), [1, 2, 3, 6]);
        assert_eq!(tree.mid_order_traversal(), [2, 1, 6, 3]);
        assert_eq!(tree.post_order_traversal(), [2, 6, 3, 1]);
    }
}
