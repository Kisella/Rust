/*
 * 原始指针的特点：
 * 1、可以忽略借用规则，允许存在多个可变或不可变指针指向同一块内存
 * 2、可能为野指针
 * 3、可能为空指针
 * 4、不会自动清理（没有drop）
 */
mod test1 {
    /*
     * case 113
     * 声明原始指针
     * *const T : 不可变原始指针，解引用后的值不能改变
     * *mut T  :   可变原始指针，解引用后的值可以改变
     */
    #[test]
    fn test_113() {
        // 通过引用来获取原始指针
        let mut num = 5;
        let r1 = &raw const num;
        let r2 = &raw mut num;

        // 原始指针不保证内存有效性
        let address = 0x12345usize;
        let r3 = address as *const i32;

        // 对原始指针解引用需要在unsafe块中进行
        unsafe {
            *r2 += 1;
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2);
            println!("r3 is {}", *r3);
        }
    }
}

/*
 * case 114
 * 调用unsafe函数
 * fn关键字前存在unsafe关键字标识的函数称为unsafe函数，对此类函数的调用需要在unsafe块中进行
 */ 
mod test2 {
    use core::slice;

    //   此函数与core::slice::split_at_mut实现了相同的功能
    fn split_at_mut1(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        assert!(mid <= values.len());

        // 此处，Rust的借用检查器无法理解我们借用的是切片的不同部分，它只知道我们对同一个切片进行了两次可变借用，故拒绝了这项代码，即使这段代码实际上是安全的
        // (&mut values[..mid], &mut values[mid..])   // compiling error

        // 通过as_mut_ptr方法来获取切片的可变原始指针
        let ptr = values.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), values.len() - mid),
            )
        }
    }

    #[test]
    fn case_114() {
        let mut v = vec![1,2,3,4,5,6];
        let r = &mut v[..];
        // let (a, b) = r.split_at_mut(3);
        let (a, b) = split_at_mut1(r,3);
        assert_eq!(a, &mut[1,2,3]);
        assert_eq!(b, &mut[4,5,6]);
    }
}

/*
 * case 115
 * 使用extern关键字调用外部函数接口
 */ 
mod test3 {
    unsafe  extern "C" {
        safe fn abs(input :i32) -> i32;         //  可使用safe关键字标识外部接口是安全的，这样就允许在unsafe块之外来调用这些接口
    }

    #[test]
    fn test_115 () {
        assert_eq!(abs(-3), 3);
    }
}

