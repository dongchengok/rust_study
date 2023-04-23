#[derive(Debug)]
struct TestObj {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct Test {
    c: i32,
    d: i32,
}

trait OpBase {
    fn op(&self) -> i32;
}

trait OpMut: OpBase {
    fn op(&self) -> i32;
}

impl OpBase for TestObj {
    fn op(&self) -> i32 {
        self.a + self.b
    }
}

impl OpMut for TestObj {
    fn op(&self) -> i32 {
        self.a * self.b
    }
}

impl OpBase for Test {
    fn op(&self) -> i32 {
        self.c + self.d
    }
}

impl OpMut for Test {
    fn op(&self) -> i32 {
        self.c * self.d * 2
    }
}

fn op_func<T: OpBase>(val: &T) -> i32 {
    val.op()
}

fn op_func_pointer_super(val: &dyn OpBase){

}

fn op_func_pointer(val: &dyn OpMut) {
    println!("op_func_pointer_base:{}",OpMut::op(val));
    //op_func_pointer_super(val);
}

fn main() {
    let obj = TestObj { a: 4, b: 8 };
    println!("OpBase:{}", OpBase::op(&obj));
    println!("OpMut:{}", OpMut::op(&obj));
    println!("OpFunc:{}", op_func(&obj));

    let ptr_op_base: Box<dyn OpBase> = Box::new(TestObj { a: 3, b: 4 });
    println!("ptr OpBase:{}", ptr_op_base.op());

    let ptr_op_mut: Box<dyn OpMut> = Box::new(TestObj { a: 3, b: 4 });
    println!("ptr OpMut:{}", OpMut::op(&*ptr_op_mut));
}
