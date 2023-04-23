use  std::collections::HashMap;

fn main(){
    let mut a : HashMap<i32, &str> = HashMap::new();
    a.insert(37,"asdf");
    a.insert(4,"fdsa");
    println!("hash_map:{:?}",a);
}