/// - 参数
/// array: &[T],              待查找的目标数组，需已完成排序
/// is_blue: Fn(&T) -> bool,  判别元素是否属于蓝色区域的闭包，用于识别红蓝边界, 蓝色区域为低索引区域
/// choose_blue: bool,        目标元素处在红蓝边界的哪一边
/// - 返回值
/// Option<usize>             目标元素的索引值，失败返回None
pub fn binary_search<T: PartialOrd, F: Fn(&T) -> bool>(
    array: &[T],
    is_blue: F,
    choose_blue: bool,
) -> Option<usize> {
    let (mut l, mut r) = (std::usize::MAX, array.len()); // 使用MAX来指代-1
    if r == 0 {
        return None;
    }

    while l.wrapping_add(1) != r {      //  调用wrapping实现回绕运算
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
