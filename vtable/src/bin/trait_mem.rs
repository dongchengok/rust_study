struct TestObj {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

trait Operate {
    fn op(&self) -> i64;
}

impl Operate for TestObj {
    fn op(&self) -> i64 {
        self.a + self.b + self.c + self.d
    }
}

impl Drop for TestObj {
    fn drop(&mut self) {
        println!("drop!!!!");
    }
}

fn dump_object_vtable(obj:TestObj){
    println!("obj op:{} size:{}", obj.op(), std::mem::size_of_val(&obj));

    //获取一个trait指针
    let ref_trait = &obj as &dyn Operate;
    println!(
        "trait op:{} size:{}",
        ref_trait.op(),
        std::mem::size_of_val(&ref_trait)
    );

    //对比一下对象地址和trait的地址
    let addr_obj: usize = unsafe { std::mem::transmute(&obj) };
    let addr_trait: usize = unsafe { std::mem::transmute(&ref_trait) };
    let mem_ptr_trait: *const usize = unsafe { std::mem::transmute(&ref_trait) };
    let addr_trait_obj: usize = unsafe { mem_ptr_trait.read() };
    let addr_trait_vtable: usize = unsafe { mem_ptr_trait.offset(1).read() };
    println!("object addr:{:X}", addr_obj);
    println!("trait addr:{:X}", addr_trait);
    println!("trait obj addr:{:X}", addr_trait_obj);
    println!("trait vtable addr:{:X}", addr_trait_vtable);

    //vtable里的内容
    //drop函数
    //size of TestObject
    //minimum alignment of TestObject
    let vtable: *const usize = unsafe { std::mem::transmute(addr_trait_vtable) };
    unsafe {
        println!("drop pointer addr:{:X}", vtable.read() as usize);
        println!(
            "vtable-sizeof TestObject:{}, size_of:{}",
            vtable.offset(1).read() as usize,
            std::mem::size_of_val(&obj)
        );
        println!(
            "vtable-aligen of TestObjet:{}, align_of:{}",
            vtable.offset(2).read() as usize,
            std::mem::align_of_val(&obj)
        );
        println!("vtable-Operator::op:{:X}", vtable.offset(3).read() as usize);
    }

    //这个地址指向的
    let op_func_addr: usize = unsafe { std::mem::transmute(TestObj::op as *const ()) };
    println!("op func addr:{:X}",op_func_addr);
    let op_in_func_ref = &TestObj::op;
    let op_ref_mem: *const usize = unsafe { std::mem::transmute(&op_in_func_ref) };
    unsafe{ println!("op ref mem:{:X}",op_ref_mem.read() as usize); }
    let drop_addr: usize = unsafe { std::mem::transmute(drop::<&dyn Operate> as *const ()) };
    println!("drop func addr:{:X}", drop_addr);
}

fn main() {
    let obj = TestObj {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    dump_object_vtable(obj);

    // let obj = TestObj {
    //     a: 1,
    //     b: 2,
    //     c: 3,
    //     d: 4,
    // };
    // dump_object_vtable(obj);
}
