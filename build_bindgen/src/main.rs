include!("bindings.rs");

fn main() {
    let ret;
    unsafe{ ret = c_add(100,2) as i32; }

    println!("Hello, world!{}",ret);
}
