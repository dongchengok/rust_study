trait Super: AsDynSuper {
    fn op(self:&Self)->i32;
}

trait AsDynSuper {
    fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
    where
        Self: 'a;
}

impl<T: Super + Sized> AsDynSuper for T {
    fn as_dyn_super<'a>(self: Box<Self>) -> Box<dyn Super + 'a>
    where
        Self: 'a,
    {
        self
    }
}

trait Sub: Super {}

struct Object1{
    a:i32,
    b:i32,
}

struct Object2{
    a:i32,
    b:i32,
}

impl Super for Object1{
    fn op(&self)->i32{
        self.a+self.b
    }
}

impl Sub for Object1{

}

impl Super for Object2{
    fn op(&self)->i32{
        self.a*self.b
    }
}

impl Sub for Object2{
    
}

fn main(){
    let obj1:Box<dyn Sub> = Box::new(Object1{a:10,b:20});
    let obj2:Box<dyn Sub> = Box::new(Object2{a:10,b:20});
    let objs = vec![obj1.as_dyn_super(),obj2.as_dyn_super()];
    for obj in objs {
        println!("op value:{}",obj.op());
    }
}