use implementation::Impl;

pub trait ImplRef<'i, T> {
    fn from_impl(_impl: &'i Impl<T>) -> Self;

    fn as_impl(&self) -> &'i Impl<T>;
}

pub trait BorrowImplRef<'i, T> {
    type Ref: ImplRef<'i, T>;
}

pub trait BorrowImpl<T>: for<'i> BorrowImplRef<'i, T> {}
