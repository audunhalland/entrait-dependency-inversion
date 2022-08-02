use core::Facade;
use core::FacadeSelectImpl;

use middle::MyImpl;
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

fn main() {
    let app = Impl::new(App);

    let num = app.foo();

    println!("Got {}", num);
}
