#[derive(Debug)]
#[allow(dead_code)]
struct Test
{
    a : i32,
    b : i64,
}

fn main() {
    let a = Test{a:1,b:2};
    println!("Test:{:?}",a);
}
