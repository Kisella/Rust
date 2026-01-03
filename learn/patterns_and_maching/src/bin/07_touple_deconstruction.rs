fn main() {
    let touple = (1,2,3);

    match touple { 
        (1, y,z) => println!("Start with 1: ({y} {z})"),
        _ => ()
    }

    match touple {
        (_, _, z) => println!("End with {z})"),
    }

    match touple {
        (.., z) => println!("End with {z})"),
    }

    let touple = (1, 2, 3, 4, 5);
    
    match touple {
        (a,b, ..) => println!("Start with {a} {b}"),
    }

    match touple {
        (.., d, e,f) => println!("End with {d} {e} {f}"),
    }

    match touple {
        (a,.., f) => println!("Start with {a}, End with {f}"),
    }

    match touple {
        /* 每层最多使用一次..,否则将导致歧义
        (.., mid, ..) => println!("The touple has {mid}"),
         */
        (_, _, mid, ..) => println!("The touple has {mid}"),    //  _和..配合使用，避免歧义
    }
}