use std::hash::Hash;

fn main() {

}

struct Student {
    name: String,
    id: u32,
    age: u32,
    class: String,
}

impl Hash for Student {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state); // 只使用 id 字段进行哈希计算
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id // 只比较 id 字段
    }
}

impl Eq for Student {}