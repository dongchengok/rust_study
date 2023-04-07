extern { fn hello(); }

fn main() {
    unsafe { hello(); }
    println!("heihei call hello!");
}