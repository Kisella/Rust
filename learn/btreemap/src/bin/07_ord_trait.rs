use std::collections::BTreeMap;
use std::cmp::Ordering;
struct Student {
    name: String,
    id: u32,
    age: u32,
    class: String,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Student {}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    let student1 = Student {
        name: "Alice".to_string(),
        id: 1001,
        age: 20,
        class: "Math".to_string(),
    };
    let map = BTreeMap::from([(student1, "Excellent")]);
}