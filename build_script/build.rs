fn main() {
    println!("cargo:rerun-if-changed=src/hello.c");

    let mut builder: cc::Build = cc::Build::new();
    builder
        .file("./src/hello.c")
        .define("SAY_HELLO", "SAY_HELLO")
        .shared_flag(false)
        .compile("hello");
    // panic!("target:{:?}",builder.target(target))
    // panic!("tools:{:?}", builder.get_compiler().path());
}
