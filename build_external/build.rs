use std::env;

fn main() {
    //这个lib是msvc cl.exe lib.exe编译的x86的
    //需要用cargo run --target=i686-pc-windows-msvc 运行
    println!("cargo:rustc-link-search=native={}", env::current_dir().unwrap().to_str().unwrap());
    println!("cargo:rustc-link-lib=static=hello");
}