#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello, world!");
}

#[no_mangle]
pub static HAHA: i32 = 1;

#[no_mangle]
pub extern "C" fn say_haha() {
    println!("say {}", HAHA);
}
