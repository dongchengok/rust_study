use std::mem::{size_of};
use std::*;

fn haha(){
    println!("haha");
}

struct TestObj {
    a : i32,
    b : i32,
}

trait TestTrait {
    fn op(&self)->i32;
}

impl TestTrait for TestObj{
    fn op(&self)->i32{
        self.a + self.b
    }
}

fn main() {
    println!("TestObj ref size:{}",size_of::<&TestObj>());
    println!("TestObj pointer size:{}",size_of::<*const TestObj>());
    println!("TestTrait ref size:{}",size_of::<&dyn TestTrait>());
    println!("TestTrait pointer size:{}",size_of::<*const dyn TestTrait>());
    println!("TestSlice size:{}",size_of::<&[i32]>());

    println!("TestObj ref ref size:{}",size_of::<&&TestObj>());
    println!("TestObj pointer ref size:{}",size_of::<&*const TestObj>());
    println!("TestTrait ref ref size:{}",size_of::<&&dyn TestTrait>());
    println!("TestTrait pointer ref size:{}",size_of::<&*const dyn TestTrait>());
    println!("TestSlice ref size:{}",size_of::<&&[i32]>());

    println!("func size:{}",size_of::<*const ()>());
}