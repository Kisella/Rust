#[repr(C)]
union MyUnion {
    i: i32,
    f: f32, 
}

fn main() {
    let u = MyUnion { f: 3.14 };
    
    unsafe {
        println!("i = {}", u.i);    //  ❌ 用错误的类型解释数据
        println!("f = {}", u.f);
    }
}