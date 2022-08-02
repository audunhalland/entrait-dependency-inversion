#![cfg_attr(feature = "nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]

use intermediate::SomeDep;

use implementation::Impl;

struct App {
    dyn_impl: intermediate::foo_dyn::MyImpl,
}

impl SomeDep for App {
    fn bar(&self) -> i32 {
        42
    }
}

impl domain::foo1::DelegateFoo1<Self> for App {
    type By = intermediate::foo1::MyImpl;
}

impl<'i> domain::foo2::DelegateFoo2<'i, Self> for App {
    type ByRef = intermediate::foo2::Foo2Ref<'i, Self>;
}

impl domain::foo3::DelegateFoo3<Self> for App {
    type By = intermediate::foo3::MyImpl;
}

impl std::borrow::Borrow<dyn domain::foo_dyn::FooDynImpl<Self>> for App {
    fn borrow(&self) -> &dyn domain::foo_dyn::FooDynImpl<Self> {
        &self.dyn_impl
    }
}

#[cfg(feature = "nightly")]
impl domain::foo_static_async::DelegateFooStaticAsync<Self> for App {
    type By = intermediate::foo_static_async::MyImpl;
}

#[tokio::main]
async fn main() {
    use domain::foo1::Foo1;
    use domain::foo2::Foo2;
    use domain::foo3::Foo3;
    use domain::foo_dyn::FooDyn;

    let app = Impl::new(App {
        dyn_impl: intermediate::foo_dyn::MyImpl,
    });

    println!("foo1: {}", app.foo1());
    println!("foo2: {}", app.foo2());
    println!("foo3: {}", app.foo3());
    println!("foo_dyn: {}", app.foo_dyn());

    #[cfg(feature = "nightly")]
    {
        use domain::foo_static_async::FooStaticAsync;
        println!("foo_static_async: {}", app.foo_static_async().await);
    }
}
