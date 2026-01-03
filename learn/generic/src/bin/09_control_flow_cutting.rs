fn main() {
    let x = 10;
    if x == 0 {
        println!("x is zero"); // 编译器识别到x不可能为0，不可能进入该分支，在生成汇编代码时直接将该分支裁剪掉。  
    } else {
        println!("x is not zero");
    }

    /* 裁剪后代码：
    let x = 10;
    println!("x is not zero");
     */
}

fn side_effect_true(buf: &mut [u8]) {
    buf[0] = 1;
}

fn side_effect_false(buf: &mut [u8]) {
    buf[0] = 2;
}

// 因为FLAG在编译器就能确定，编译器会裁剪掉不会执行的分支
fn test<const FLAG: bool>(buf: &mut [u8]) {
    if FLAG {
        side_effect_true(buf);
    } else {
        side_effect_false(buf);
    }
}

// flag在运行时才能确定，编译器未能进行裁剪
fn test_runtime(flag: bool, buf: &mut [u8]) {
    if flag {
        side_effect_true(buf);
    } else {
        side_effect_false(buf);
    }
}