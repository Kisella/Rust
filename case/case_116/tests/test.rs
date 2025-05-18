/*
 * Rust中的全局变量称为静态(static)变量
 * 静态变量只能储存具有'static生命周期的引用
 */


/*
 * case 116
 * 声明并读取静态变量
 * 使用static关键字来声明静态变量
 * 对静态变量的读取或修改须在unsafe代码块中进行
 */ 
mod test_116 {
    static mut COUNTER:u32 = 0;
    unsafe fn add_to_count(inc: u32)  {
        unsafe {
            COUNTER += inc;
        }
    }
    #[test]
    fn test_116() {
        unsafe {
            add_to_count(3);
            // Rust不允许为静态变量创建可变引用或不可变引用
            // println!("COUNTER: {}", COUNTER);   // Compiling error
            
            // 创建静态变量的raw指针来对其进行访问
            println!("COUNTER: {}", *(&raw const COUNTER));

        }
    }
}