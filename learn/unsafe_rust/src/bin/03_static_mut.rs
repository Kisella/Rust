static mut NUM: i32 = 10;

fn main() {
    unsafe {
        NUM += 5;
        // println!("NUM: {}", NUM);  // ❌ 不允许创建引用
        let p1 = &raw const NUM;  // ✅ 创建原始指针
        println!("NUM: {}", *p1);       //  通过原生指针访问值
    }
}