use std::mem::{size_of,size_of_val};
use std::*;

fn haha(){
    println!("haha");
}

struct TestObj {
    a : i64,
    b : i64,
    c : i64,
    d : i64,
}

trait TestTrait {
    fn op(&self)->i64;
}

impl TestTrait for TestObj{
    fn op(&self)->i64{
        self.a + self.b
    }
}

fn main(){
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    println!("sizeof slice:{}",size_of_val(&part));
    //按reference说法，后面紧跟的是slice长度，我们验证一下
    // println!("lenth of slice:{}",part )
}