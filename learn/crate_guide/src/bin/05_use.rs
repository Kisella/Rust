mod m1 {
    pub mod sub_m1 {
        use super::super::m2::SM;
        pub fn function(zero: SM, one :SM) {
            println!("Ask for help, please call {one:?}, {one:?}, {zero:?}");
        }
    }
}

mod m2 {
    #[derive(Debug)]
    pub enum SM {
        ZERO,
        ONE,
    }
}

use m1::sub_m1::*;
use m2::SM::*;

fn main() {
    let kiana = ZERO;   //  可以直接使用变体名
    let mei = ONE;
    function(kiana, mei); // 可以直接使用函数名调用
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    // -- snip--
    IoResult::Ok(())
}
