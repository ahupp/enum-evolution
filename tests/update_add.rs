use enum_evolution::enum_evolution;

enum_evolution! {
    pub enum Foo {
        Zero,
        One(u32),
    }

    derive Bar from Foo {
        update One(u64);
        add Two(String);
    }
}

#[test]
fn can_use_updated_and_added_variants() {
    let _ = Bar::One(42u64);
    let _ = Bar::Two(String::from("hello"));
}
