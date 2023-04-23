use std::mem::{size_of_val,size_of,align_of};
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
    println!("func size:{}",size_of::<*const ()>());
    //println!("TestTrait pointer size:{}",mem::size_of::<*const dyn TestTrait>());

    let func = &haha;
    let hehe = &func;
    let a = 1i32;
    let b = &a;
    println!("func size:{}",size_of_val(&func));
    println!("func size:{}",size_of_val(func));
    println!("i32 size:{}",size_of_val(b));
    println!("Hello, world!");

    println!("func:{:?}",ptr::addr_of!(func));
    println!("a:{:?}",ptr::addr_of!(a));
    println!("b:{:?}",ptr::addr_of!(b));

    let phaha:u64 = unsafe{ std::mem::transmute(haha as *const ()) };
    let p:u64 = unsafe{ std::mem::transmute(&func) };
    let p1:u64 = unsafe{ std::mem::transmute(&hehe) };
    let aa:i32 = unsafe { std::mem::transmute(a)};
    let pa:u64 = unsafe { std::mem::transmute(&a)};
    let pb:u64 = unsafe { std::mem::transmute(b)};
    let ppb:u64 = unsafe { std::mem::transmute(&b)};
    //let xxx = unsafe{ (haha as *const ()).offset(1) };
    println!("haha:{:X}",phaha);
    println!("func:{:X}",p);
    println!("hehe:{:X}",p1);
    println!("pa:{}",aa);
    println!("pa:{:X}",pa);
    println!("pb:{:X}",pb);
    println!("ppb:{:X}",ppb);
    let aa = 0;
}
