// #[no_mangle]
// pub extern "C" fn hello_from_rust() {
//     println!("Hello from Rust! no_mangle");
// }

// #[export_name = "hello_from_xxx"]
// pub extern "C" fn hello_from_rust1() {
//     println!("Hello from Rust!xxx");
// }
#[no_mangle]
pub extern "C" fn foo(a: i32, b: i32) -> i32{
    // println!("hello : a + b = {}", a + b);
    a+b
}
