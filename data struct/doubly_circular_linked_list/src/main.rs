use doubly_circular_linked_list::model::*;

fn main() {
    let mut list = List::new();
    list.append(0);
    list.append(1);
    list.append(2);
    list.append(3);
    println!("{list:?}");
    assert_eq!(list.index(0).unwrap(), 0);
    assert_eq!(list.index(1).unwrap(), 1);
    assert_eq!(list.index(2).unwrap(), 2);
    assert_eq!(list.index(3).unwrap(), 3);
    list.delete(0).unwrap();
    assert_eq!(list.index(0).unwrap(), 1);
    list.delete(2).unwrap();
    assert_eq!(list.index(1).unwrap(), 2);
    println!("{list:?}");
}
