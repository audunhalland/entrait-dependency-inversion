use framework::ImplRef;

use implementation::Impl;

pub mod foo1 {
    use super::*;

    pub trait Foo1 {
        fn foo1(&self) -> i32;
    }

    pub trait Foo1Impl<T> {
        fn foo1(__self: &Impl<T>) -> i32;
    }

    pub trait Foo1SelectImpl<T> {
        type Impl: Foo1Impl<T>;
    }

    // Delegation
    impl<T> Foo1 for Impl<T>
    where
        T: Foo1SelectImpl<T>,
    {
        fn foo1(&self) -> i32 {
            <T as Foo1SelectImpl<T>>::Impl::foo1(self)
        }
    }
}

pub mod foo2 {
    use super::*;

    pub trait Foo2 {
        fn foo2(&self) -> i32;
    }

    pub trait SelectFoo2<'i, T> {
        type Ref: Foo2 + framework::ImplRef<'i, T>;
    }

    impl<T> Foo2 for Impl<T>
    where
        T: for<'i> SelectFoo2<'i, T>,
    {
        fn foo2(&self) -> i32 {
            <T as SelectFoo2<T>>::Ref::from_impl(self).foo2()
        }
    }
}

pub mod foo3 {
    use super::*;

    pub trait Foo3 {
        fn foo3(&self) -> i32;
    }

    pub trait SelectFoo3<T> {
        type By: framework::BorrowImpl<T>;
    }

    impl<T> Foo3 for Impl<T>
    where
        T: SelectFoo3<T>,
        for<'i> <T::By as framework::BorrowImplRef<'i, T>>::Ref: Foo3 + framework::ImplRef<'i, T>,
    {
        fn foo3(&self) -> i32 {
            <T::By as framework::BorrowImplRef<T>>::Ref::from_impl(self).foo3()
        }
    }
}
