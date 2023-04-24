struct Object {
    a: i32,
    b: i32,
}

trait Operate {
    fn op(&self) -> i32;
}

impl Operate for Object {
    fn op(&self) -> i32 {
        self.a + self.b
    }
}

fn main() {
    let obj = Object { a: 10, b: 20 };
    let obj_ref: &dyn Operate = &obj;
    obj.op();
    obj_ref.op();
}
