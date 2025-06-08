pub fn binary_search<T: PartialOrd, F: Fn(&T) -> bool>(
    array: &[T],
    is_blue: F,
    choose_blue: bool,
) -> Option<usize> {
    let (mut l, mut r) = (std::usize::MAX, array.len()); // 使用MAX来指代-1
    if r == 0 || !array.is_sorted() {
        return None;
    }

    //  调用wrapping实现回绕运算
    while l.wrapping_add(1) != r {
        let m = l.wrapping_add(r) / 2;
        if is_blue(&array[m]) { l = m } else { r = m }
    }
    if choose_blue && l != std::usize::MAX {
        Some(l)
    } else if !choose_blue && r != array.len() {
        Some(r)
    } else {
        None
    }
}
