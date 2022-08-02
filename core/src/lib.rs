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

/// IMPL 2:

/// Library start
pub trait ImplRef<'i, T> {
    fn from_impl(_impl: &'i Impl<T>) -> Self;

    fn as_impl(&self) -> &'i Impl<T>;
}
pub trait Proxy<T> {
    type Ref: for<'i> ImplRef<'i, T>;
}
/// Library end

pub trait Foo2 {
    fn foo2(&self) -> i32;
}

pub trait SelectFoo2<'i, T> {
    type Ref: Foo2 + ImplRef<'i, T>;
}

impl<T> Foo2 for Impl<T> where T: for<'i> SelectFoo2<'i, T> {
    fn foo2(&self) -> i32 {
        <T as SelectFoo2<T>>::Ref::from_impl(self).foo2()
    }
}
