fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };

    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x: a, y: 0 } => println!("On x-axit: {a}"),
        Point { x: 0, y: b } => println!("On y-axit: {b}"),
        Point { x: a, y: b } => println!("On neither axit: ({a} {b})"),
    }

    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y: 0 } => println!("On x-axit: {x}"),
        Point { x: 0, y } => println!("On y-axit: {y}"),
        Point { x, y } => println!("On neither axit: ({x} {y})"),
    }

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let p: Point3D = Point3D { x: 1, y: 2, z: 3 };

    match p {
        Point3D { x: 0, y: 0, z } => println!("z is {z}"),
        Point3D { x: 0, y, .. } => println!("y is {y}"),
        Point3D { x, y: 0, .. } => println!("x is {x}"),
        Point3D { z, ..} => println!("x and y both not zero, z is {z}"),
    }
}
