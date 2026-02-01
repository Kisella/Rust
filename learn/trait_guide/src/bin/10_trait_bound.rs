use std::fmt::Display;



// fn get_max<T> (a: T, b: T) -> T {
//     if a > b { a } else { b } //  编译失败，不是任何类型都支持比较大小。
// }

fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b } //  编译失败，不是任何类型都支持比较大小。
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    fn show(&self) {
        println!("x = {}, y = {}", self.x, self.y);
    }
}

impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    fn add(&self, other: &Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Holder<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> Holder<T> {
    pub fn print_value(&self) {
        println!("value is {}", self.value);
    }
}

trait Summary<T: std::fmt::Display> {
    fn summarize(&self, item: T) -> String;
}

struct News;

impl Summary<String> for News {
    fn summarize(&self, item: String) -> String {
        format!("新闻摘要：{item}")
    }
}

trait Container {
    type Item: Display;
}
impl Container for News {
    type Item = String;
}

trait Team{
    type Leader;
}

trait Speak {
    fn say_hello(&self);
}

fn interview<T>(team: T, leader: <T as Team>::Leader)
where
    T: Team,
    <T as Team>::Leader: Speak,
{
    leader.say_hello();
}

fn main() {

}