pub trait MyTrait {
    fn a(&self);
}

pub struct MyStruct {}

impl MyTrait for MyType {
    fn a(&self) {
        unimplemented!()
    }
}

pub fn my_fn() {
    unimplemented!()
}
