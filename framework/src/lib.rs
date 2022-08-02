use implementation::Impl;

pub trait ImplRef<'i, T> {
    fn from_impl(_impl: &'i Impl<T>) -> Self;

    fn as_impl(&self) -> &'i Impl<T>;
}

pub trait Proxy<T> {
    type Ref: for<'i> ImplRef<'i, T>;
}
