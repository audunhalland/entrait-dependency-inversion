use core::Facade;
use core::Foo2;
use core::SelectFoo2;
use core::FacadeSelectImpl;

use middle::MyImpl;
use middle::Foo2Ref;
use middle::SomeDep;

use implementation::Impl;

struct App;

impl SomeDep for App {
    fn bar(&self) -> i32 {
        42
    }
}

impl FacadeSelectImpl<Self> for App {
    type Impl = MyImpl;
}

impl<'i> SelectFoo2<'i, Self> for App {
    type Ref = Foo2Ref<'i, Self>;
}

fn main() {
    let app = Impl::new(App);

    let num = app.foo();

    println!("Impl1: {}", num);

    let num2 = app.foo2();

    println!("Impl2: {}", num2);
}
