use std::mem::{size_of, size_of_val, transmute};
use std::*;

fn haha() {
    println!("haha");
}

struct TestObj {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

trait TestTrait {
    fn op(&self) -> i64;
}

impl TestTrait for TestObj {
    fn op(&self) -> i64 {
        self.a + self.b
    }
}

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[1..4];
    println!("sizeof slice:{}", size_of_val(&part));

    let addr: usize = unsafe { transmute(&part) };
    println!("addr of part:{:x}", addr);
    //取一下part变量的地址
    let addr_len: *const usize = unsafe { transmute(addr + 8) };
    println!("addr of slice length:{:?}", addr_len);
    //内存里取到的slice的实际长度是
    let len = unsafe { *addr_len };
    println!("length in mem:{}", len);

    //在看一下数组的内存地址
    let arr_addr: *const i32 = unsafe { transmute(&arr) };
    println!("arr addr:{:?}", arr_addr);
    //他应该等于切片数据块第一个指针指向的地址加切片的偏移
    let slice_ptr_addr: *const usize = unsafe { transmute(addr) };
    unsafe {
        println!("slice to arr addr:{:?}", *slice_ptr_addr);
    };
    //切片指向的地址
    let slice_data_addr: *const i32 = unsafe { transmute(*slice_ptr_addr) };
    println!("slice data addr:{:?}",slice_data_addr);

    //根据地址输出数组数据
    for i in 0..arr.len() {
        unsafe {
            println!("arr[{}]:{}", i, arr_addr.offset(i as isize).read() as i32);
        }
    }

    //根据数组输出slice数据
    for i in 0..part.len() {
        unsafe {
            println!(
                "slice[{}]:{}",
                i,
                slice_data_addr.offset(i as isize).read() as i32
            );
        }
    }
}
