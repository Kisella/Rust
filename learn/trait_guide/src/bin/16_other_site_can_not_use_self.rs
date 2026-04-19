trait Test {
    fn merge(&self, other: &Self);    
}

// ❌ 对于func函数来说，它只保证t1和t2是实现了Test的类型，但不能保证t1和t2最终指向了相同的类型
// fn fun(t1: &dyn Test, t2: &dyn Test) {
//     t1.merge(t2);
// }


fn main() {

}