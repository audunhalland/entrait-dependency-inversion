use middle::SomeDep;

use implementation::Impl;

struct App {
    dyn_impl: middle::foo_dyn::MyImpl,
}

impl SomeDep for App {
    fn bar(&self) -> i32 {
        42
    }
}

impl core::foo1::DelegateFoo1<Self> for App {
    type By = middle::foo1::MyImpl;
}

impl<'i> core::foo2::DelegateFoo2<'i, Self> for App {
    type ByRef = middle::foo2::Foo2Ref<'i, Self>;
}

impl core::foo3::DelegateFoo3<Self> for App {
    type By = middle::foo3::MyImpl;
}

impl std::borrow::Borrow<dyn core::foo_dyn::FooDynImpl<Self>> for App {
    fn borrow(&self) -> &dyn core::foo_dyn::FooDynImpl<Self> {
        &self.dyn_impl
    }
}

fn main() {
    use core::foo1::Foo1;
    use core::foo2::Foo2;
    use core::foo3::Foo3;
    use core::foo_dyn::FooDyn;

    let app = Impl::new(App {
        dyn_impl: middle::foo_dyn::MyImpl,
    });

    println!("foo1: {}", app.foo1());
    println!("foo2: {}", app.foo2());
    println!("foo3: {}", app.foo3());
    println!("foo_dyn: {}", app.foo_dyn());
}
