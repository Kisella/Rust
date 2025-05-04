use crate::models::structure::*;

impl<T: PartialOrd + Clone> PriorityQueue<T> {
    // 自下而上建队法。对每个父节点进行下滤操作
    // 从倒数第二排的最后一个父节点开始(index: (len)/2 - 1)
    pub fn create_max_heap(mut elements: Vec<T>) -> Vec<T> {
        if elements.is_empty() {
            return elements;
        }
        let border = elements.len() - 1;
        for mut i in (0..border / 2).rev() {
            while i < border / 2 {
                if 2 * i + 1 <= border
                    && elements[i] < elements[2 * i + 1]
                    && (2 * i + 2 > border || elements[2 * i + 2] < elements[2 * i + 1])
                {
                    elements.swap(i, 2 * i + 1);
                    i = 2 * i + 1;
                } else if 2 * i + 2 <= border && elements[i] < elements[2 * i + 2] {
                    elements.swap(i, 2 * i + 2);
                    i = 2 * i + 2;
                } else {
                    break;
                }
            }
        }
        elements
    }

    pub fn create_mim_heap(mut elements: Vec<T>) -> Vec<T> {
        let border = elements.len() - 1;
        for mut i in (0..border / 2).rev() {
            while i < border / 2 {
                if 2 * i + 1 <= border
                    && elements[i] > elements[2 * i + 1]
                    && (2 * i + 2 > border || elements[2 * i + 2] > elements[2 * i + 1])
                {
                    elements.swap(i, 2 * i + 1);
                    i = 2 * i + 1;
                } else if 2 * i + 2 <= border && elements[i] > elements[2 * i + 2] {
                    elements.swap(i, 2 * i + 2);
                    i = 2 * i + 2;
                } else {
                    break;
                }
            }
        }
        elements
    }

    // 上滤操作插入元素
    pub fn append_max_heap(heap: &mut Vec<T>, value: T) {
        heap.push(value);
        let mut i = heap.len() - 1;
        while i > 0 && heap[i] > heap[(i + 1) / 2 - 1] {
            heap.swap(i, (i + 1) / 2 - 1);
            i = (i + 1) / 2 - 1;
        }
    }

    pub fn append_mim_heap(heap: &mut Vec<T>, value: T) {
        heap.push(value);
        let mut i = heap.len() - 1;
        while i > 0 && heap[i] < heap[(i + 1) / 2 - 1] {
            heap.swap(i, (i + 1) / 2 - 1);
            i = (i + 1) / 2 - 1;
        }
    }

    // 优先权队列弹出
    // 弹出头元素 -> 尾元素替换为新头节点 -> 对新头节点进行下滤操作
    pub fn pop_max_heap(heap: &mut Vec<T>) -> Option<T> {
        match heap.len() {
            0 => return None,
            1 => return heap.pop(),
            _ => {},
        }
        // 交换头节点和末尾元素后弹出
        let mut border = heap.len() - 1;
        heap.swap(0, border);
        let res = heap.pop();
        border -= 1;

        // 对新的头结点进行下滤操作
        let mut i = 0;
        while i < border / 2 {
            if 2 * i + 1 <= border
                && heap[i] < heap[2 * i + 1]
                && (2 * i + 2 > border || heap[2 * i + 2] < heap[2 * i + 1])
            {
                heap.swap(i, 2 * i + 1);
                i = 2 * i + 1;
            } else if 2 * i + 2 <= border && heap[i] < heap[2 * i + 2] {
                heap.swap(i, 2 * i + 2);
                i = 2 * i + 2;
            } else {
                break;
            }
        }
        res
    }

    pub fn pop_mim_heap(heap: &mut Vec<T>) -> Option<T> {
        match heap.len() {
            0 => return None,
            1 => return heap.pop(),
            _ => {},
        }
        // 交换头节点和末尾元素后弹出
        let mut border = heap.len() - 1;
        heap.swap(0, border);
        let res = heap.pop();
        border -= 1;

        // 对新的头结点进行下滤操作
        let mut i = 0;
        while i < border / 2 {
            if 2 * i + 1 <= border
                && heap[i] > heap[2 * i + 1]
                && (2 * i + 2 > border || heap[2 * i + 2] > heap[2 * i + 1])
            {
                heap.swap(i, 2 * i + 1);
                i = 2 * i + 1;
            } else if 2 * i + 2 <= border && heap[i] > heap[2 * i + 2] {
                heap.swap(i, 2 * i + 2);
                i = 2 * i + 2;
            } else {
                break;
            }
        }
        res
    }
}
