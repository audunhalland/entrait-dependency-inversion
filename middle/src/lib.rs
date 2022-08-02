use core::FacadeImpl;

use implementation::Impl;

pub trait SomeDep {
    fn bar(&self) -> i32;
}

impl<T> SomeDep for Impl<T> where T: SomeDep {
    fn bar(&self) -> i32 {
        self.as_ref().bar()
    }
}

pub struct MyImpl;

impl<T> FacadeImpl<T> for MyImpl where Impl<T>: SomeDep {
    fn foo(__self: &Impl<T>) -> i32 {
        __self.bar()
    }
}
