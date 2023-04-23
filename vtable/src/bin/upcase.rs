use std::sync::Arc;

#[derive(Debug)]
struct TestObj {
    a: i32,
    b: i32,
}

trait AsDynSuper {
    fn as_dyn_super<'a>(self: Self) -> Box<dyn OpBase + 'a>
    where
        Self: 'a;
}

impl<T: OpBase + Sized> AsDynSuper for T {
    fn as_dyn_super<'a>(self: Self) -> Box<dyn OpBase + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}


trait OpBase : AsDynSuper{
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

// fn upcast(obj:&dyn OpMut)->Box<dyn OpBase>{
//     // obj.as_dyn_super()

// }

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

}
