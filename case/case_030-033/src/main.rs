/*
 * 为含泛型的结构实现方法
 */ 

/*
 * case 30
 * 为含有泛型参数的结构体(或枚举)实现方法
 * 方法中使用类型参数需要先在尖括号(<>)里声明并方法impl关键字后
 */

struct Point<T> {
    x: T,
    y: T,
}

// 这里对泛型参数T的两处声明都是必不可少的
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/*
 * case 31
 * 为含有泛型参数的结构体(或枚举)的某个具体的类型实现独有方法
 * 结构体名称后的尖括号里写具体的类型，impl关键字后不接任何东西，可为特定类型的结构体实现独有方法
 */
impl Point<f32> {   //  为Ponit的<f32>类型实现独有的方法
    fn y(&self) -> &f32 {
        &self.y
    }


    /*
     * case 32
     * 不能同时为泛型类型(<T>)和具体的类型(例如<f32>), 实现同一个名称的方法
     */
    // fn x(&self) -> &f32 {
    //     &self.x
    // }
}

/*
 * case 33
 * 为含有泛型参数的结构体(或枚举)实现方法时，方法(函数)可以声明新的类型参数以满足需求
 */
#[derive(Debug)]
struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    // 声明了两个新的泛型类型<X2, Y2>，与结构体Point<X1, Y1>加以区分
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("case 30");
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    println!("\ncase 31");
    // println!("p.y = {}", p.y());
    println!("Comliping error!");   //  p实例不具有y()方法，因为它不是<f32>类型的
    let q = Point {x:3.8 , y:6.3};
    println!("q.x = {}", q.x());
    println!("q.y = {}", q.y());    //  q实例同时具有x()方法和y()方法

    println!("\ncase 33");
    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{p3:#?}");
}
