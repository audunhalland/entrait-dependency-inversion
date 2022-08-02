use implementation::Impl;

pub trait Facade {
    fn foo(&self) -> i32;
}

pub trait FacadeImpl<T> {
    fn foo(__self: &Impl<T>) -> i32;
}

pub trait FacadeSelectImpl<T> {
    type Impl: FacadeImpl<T>;
}

// Delegation
impl<T> Facade for Impl<T> where T: FacadeSelectImpl<T> {
    fn foo(&self) -> i32 {
        <T as FacadeSelectImpl<T>>::Impl::foo(self)
    }
}
