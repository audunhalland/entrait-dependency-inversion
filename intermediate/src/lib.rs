#![cfg_attr(feature = "nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

use framework::ImplRef;
use implementation::Impl;

pub trait SomeDep {
    fn bar(&self) -> i32;
}

impl<T> SomeDep for Impl<T>
where
    T: SomeDep,
{
    fn bar(&self) -> i32 {
        self.as_ref().bar()
    }
}

pub mod foo1 {
    use super::*;
    use domain::foo1::Foo1Impl;

    pub struct MyImpl;

    impl<T> Foo1Impl<T> for MyImpl
    where
        Impl<T>: SomeDep,
    {
        fn foo1(__self: &Impl<T>) -> i32 {
            __self.bar()
        }
    }
}

pub mod foo2 {
    use super::*;
    use domain::foo2::Foo2;

    pub struct Foo2Ref<'i, T>(&'i Impl<T>);

    impl<'i, T> ImplRef<'i, T> for Foo2Ref<'i, T> {
        fn from_impl(_impl: &'i Impl<T>) -> Self {
            Self(_impl)
        }
        fn as_impl(&self) -> &'i Impl<T> {
            self.0
        }
    }

    impl<'i, T> Foo2 for Foo2Ref<'i, T>
    where
        Impl<T>: SomeDep,
    {
        fn foo2(&self) -> i32 {
            self.as_impl().bar()
        }
    }
}

pub mod foo3 {
    use super::*;
    use domain::foo3::Foo3;

    pub struct MyImpl;

    pub struct MyImplRef<'i, T>(&'i Impl<T>);

    impl<'i, T> framework::ImplRef<'i, T> for MyImplRef<'i, T> {
        fn from_impl(_impl: &'i Impl<T>) -> Self {
            Self(_impl)
        }
        fn as_impl(&self) -> &'i Impl<T> {
            self.0
        }
    }

    impl<'i, T: 'static> framework::BorrowImplRef<'i, T> for MyImpl {
        type Ref = MyImplRef<'i, T>;
    }

    impl<T: 'static> framework::BorrowImpl<T> for MyImpl {}

    impl<'i, T> Foo3 for MyImplRef<'i, T>
    where
        Impl<T>: SomeDep,
    {
        fn foo3(&self) -> i32 {
            self.as_impl().bar()
        }
    }
}

pub mod foo_dyn {
    use super::*;
    use domain::foo_dyn::FooDynImpl;

    pub struct MyImpl;

    impl<T> FooDynImpl<T> for MyImpl
    where
        Impl<T>: SomeDep,
    {
        fn foo_dyn(&self, _impl: &Impl<T>) -> i32 {
            _impl.bar()
        }
    }
}

#[cfg(feature = "nightly")]
pub mod foo_static_async {
    use super::*;
    use domain::foo_static_async::FooStaticAsync;
    use std::future::Future;

    pub struct MyImpl;

    pub struct MyImplRef<'i, T>(&'i Impl<T>);

    impl<'i, T> framework::ImplRef<'i, T> for MyImplRef<'i, T> {
        fn from_impl(_impl: &'i Impl<T>) -> Self {
            Self(_impl)
        }
        fn as_impl(&self) -> &'i Impl<T> {
            self.0
        }
    }

    impl<'i, T: 'static> framework::BorrowImplRef<'i, T> for MyImpl {
        type Ref = MyImplRef<'i, T>;
    }

    impl<T: 'static> framework::BorrowImpl<T> for MyImpl {}

    impl<'i, T> FooStaticAsync for MyImplRef<'i, T>
    where
        Impl<T>: SomeDep,
    {
        type Fut<'s> = impl Future<Output = i32> where Self: 's;

        fn foo_static_async(&self) -> Self::Fut<'_> {
            async move { self.as_impl().bar() }
        }
    }
}
