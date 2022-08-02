use middle::SomeDep;

use implementation::Impl;

struct App;

impl SomeDep for App {
    fn bar(&self) -> i32 {
        42
    }
}

impl core::foo1::Foo1SelectImpl<Self> for App {
    type Impl = middle::foo1::MyImpl;
}

impl<'i> core::foo2::SelectFoo2<'i, Self> for App {
    type Ref = middle::foo2::Foo2Ref<'i, Self>;
}

impl core::foo3::SelectFoo3<Self> for App {
    type By = middle::foo3::MyImpl;
}

fn main() {
    use core::foo1::Foo1;
    use core::foo2::Foo2;
    use core::foo3::Foo3;

    let app = Impl::new(App);

    println!("foo1: {}", app.foo1());
    println!("foo2: {}", app.foo2());
    println!("foo3: {}", app.foo3());
}
