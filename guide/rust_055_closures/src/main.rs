/*
 * 闭包：可以存储在变量中或作为参数传递给其他函数的匿名函数
 */
#[derive(Debug,Clone, Copy)]    // 给ShirtColor枚举添加#[derive(Debug, Clone, Copy)]属性。这样，当调用giveaway方法时，user_pref1和user_pref2会被复制而不是移动，从而允许在后续的println!中继续使用它们。
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //  使用闭包来捕获环境，本例中捕获了self的不可变引用
        user_preference.unwrap_or_else(|| self.most_stocked())   /* unwrap_or_else方法的参数就是一个闭包，该闭包没有参数(||中间为空), 函数体为self.most_stocked(), 因为unwrap_or_else的参数要求是T，故该为闭包的返回值类型就被推断为了T, 这里的T即为ShirtColor*/
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red: i32 = 0;
        let mut num_blue: i32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}
