#![cfg_attr(feature = "nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

use framework::ImplRef;

use implementation::Impl;

/// Solution 1:
///
/// Starts with `Foo1`, generates two traits from that one:
/// `Foo1Impl`: A very close copy of the trait that removes `&self` and adds `&Impl<T>`.
/// `DelegateFoo1`: This is implemented by the application type.
pub mod foo1 {
    use super::*;

    pub trait Foo1 {
        fn foo1(&self) -> i32;
    }

    pub trait Foo1Impl<T> {
        fn foo1(__self: &Impl<T>) -> i32;
    }

    pub trait DelegateFoo1<T> {
        type By: Foo1Impl<T>;
    }

    // Delegation
    impl<T> Foo1 for Impl<T>
    where
        T: DelegateFoo1<T>,
    {
        fn foo1(&self) -> i32 {
            <T as DelegateFoo1<T>>::By::foo1(self)
        }
    }
}

/// Solution 2:
///
/// This reuses the same trait, but relies on custom smart pointers implementing
/// the trait.
/// The smart-pointer implementation details are revealed through the delegation trait.
pub mod foo2 {
    use super::*;

    pub trait Foo2 {
        fn foo2(&self) -> i32;
    }

    pub trait DelegateFoo2<'i, T> {
        type ByRef: Foo2 + framework::ImplRef<'i, T>;
    }

    impl<T> Foo2 for Impl<T>
    where
        T: for<'i> DelegateFoo2<'i, T>,
    {
        fn foo2(&self) -> i32 {
            <T as DelegateFoo2<T>>::ByRef::from_impl(self).foo2()
        }
    }
}

/// Solution 3:
///
/// This reuses the same trait that relies on custom smart-pointer, but the smart-pointer
/// implementation details are removed from the delegation trait, which is mandatory to implement by hand.
pub mod foo3 {
    use super::*;

    pub trait Foo3 {
        fn foo3(&self) -> i32;
    }

    pub trait DelegateFoo3<T> {
        type By: framework::BorrowImpl<T>;
    }

    impl<T> Foo3 for Impl<T>
    where
        T: DelegateFoo3<T>,
        for<'i> <T::By as framework::BorrowImplRef<'i, T>>::Ref: Foo3,
    {
        fn foo3(&self) -> i32 {
            <T::By as framework::BorrowImplRef<T>>::Ref::from_impl(self).foo3()
        }
    }
}

/// Dynamic dispatch
pub mod foo_dyn {
    use std::borrow::Borrow;

    use super::*;

    pub trait FooDyn {
        fn foo_dyn(&self) -> i32;
    }

    pub trait FooDynImpl<T>: 'static {
        fn foo_dyn(&self, _impl: &Impl<T>) -> i32;
    }

    impl<T: 'static> FooDyn for Impl<T>
    where
        T: core::borrow::Borrow<dyn FooDynImpl<T>>,
    {
        fn foo_dyn(&self) -> i32 {
            <T as Borrow<dyn FooDynImpl<T>>>::borrow(&*self).foo_dyn(self)
        }
    }
}

#[cfg(feature = "nightly")]
pub mod foo_static_async {
    use super::*;
    use std::future::Future;

    pub trait FooStaticAsync {
        type Fut<'s>: Future<Output = i32>
        where
            Self: 's;

        fn foo_static_async(&self) -> Self::Fut<'_>;
    }

    pub trait DelegateFooStaticAsync<T> {
        type By: framework::BorrowImpl<T>;
    }

    impl<T> FooStaticAsync for Impl<T>
    where
        T: DelegateFooStaticAsync<T> + 'static,
        for<'i> <T::By as framework::BorrowImplRef<'i, T>>::Ref: FooStaticAsync,
    {
        type Fut<'s> = impl Future<Output = i32>;

        fn foo_static_async(&self) -> Self::Fut<'_> {
            async move {
                <T::By as framework::BorrowImplRef<T>>::Ref::from_impl(self)
                    .foo_static_async()
                    .await
            }
        }
    }
}
