mod case1 {
    use priority_queue::models::structure::PriorityQueue;

    #[test]
    fn case_1() {
        assert_eq!(
            PriorityQueue::create_max_heap(vec![3, 4, 5, 6, 1, 7, 8]),
            [8, 6, 7, 4, 1, 3, 5]
        );
    }

    #[test]
    fn case_2() {
        PriorityQueue::create_max_heap(Vec::<i32>::new());
        assert_eq!(PriorityQueue::create_max_heap(vec![0]), [0]);
    }

    #[test]
    fn case_3() {
        let mut priority_queue  = PriorityQueue {
            heap: Vec::<i32>::new(),
        };
        PriorityQueue::append_max_heap(&mut priority_queue.heap,3);
        assert_eq!(priority_queue.heap, [3]);
        PriorityQueue::append_max_heap(&mut priority_queue.heap,4);
        assert_eq!(priority_queue.heap, [4,3]);
        PriorityQueue::append_max_heap(&mut priority_queue.heap,5);
        assert_eq!(priority_queue.heap, [5,3,4]);
        PriorityQueue::append_max_heap(&mut priority_queue.heap,6);
        assert_eq!(priority_queue.heap, [6,5,4,3]); 
        PriorityQueue::append_max_heap(&mut priority_queue.heap,1);
        assert_eq!(priority_queue.heap, [6,5,4,3,1]); 
        PriorityQueue::append_max_heap(&mut priority_queue.heap,7);
        assert_eq!(priority_queue.heap, [7,5,6,3,1,4]); 
        PriorityQueue::append_max_heap(&mut priority_queue.heap,8);
        assert_eq!(priority_queue.heap, [8,5,7,3,1,4,6]); 
    }

    #[test]
    fn case_4() {
        let mut priority_queue = PriorityQueue {
            heap: PriorityQueue::create_mim_heap(vec![6,10,3,12,1,2,5]),
        };
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 1);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 2);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 3);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 5);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 6);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 10);
        assert_eq!(PriorityQueue::pop_mim_heap(&mut priority_queue.heap).unwrap(), 12);
    }
}
