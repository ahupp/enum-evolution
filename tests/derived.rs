use enum_evolution::enum_evolution;

enum_evolution! {
    pub enum Foo {
        Zero,
        One(usize),
    }

    derive Bar from Foo {
        remove Zero
    }
}

#[test]
fn can_use_derived_enum() {
    let _ = Bar::One(42);
}
