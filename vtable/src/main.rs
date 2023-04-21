use std::mem::{size_of_val,size_of};
use std::*;

// trait Super {}

// trait Sub: Super {}

// fn upcast(obj: Box<dyn Sub>) -> Box<dyn Super> {
//     obj
// }

// trait Super: AsDynSuper {}

// trait AsDynSuper {
//     fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
//     where
//         Self: 'a;
// }

// impl<T: Super + Sized> AsDynSuper for T {
//     fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
//     where
//         Self: 'a,
//     {
//         self
//     }
// }

// trait Sub: Super {}

// fn upcast(obj: Box<dyn Sub>) -> Box<dyn Super> {
//     obj.as_dyn_super()
// }

fn haha(){
    println!("haha");
}

fn main() {
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
