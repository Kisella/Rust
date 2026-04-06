trait Test {
    fn merge(&self, other: &Self);
}

fn fun(t1: &dyn Test, t2: &dyn Test) {
    t1.merge(t2);
}

fn main() {

}