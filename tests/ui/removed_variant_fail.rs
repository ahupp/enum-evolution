use enum_evolution::enum_evolution;

enum_evolution! {
    enum Foo {
        Zero,
        One(usize),
    }

    derive Bar from Foo {
        remove Zero
    }
}

fn main() {
    let _ = Bar::Zero;
}
