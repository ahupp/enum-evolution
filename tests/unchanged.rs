use enum_evolution::enum_evolution;

enum_evolution! {
    pub enum Example {
        A,
        B(u32),
    }
}

#[test]
fn allows_using_enum() {
    let _ = Example::A;
}
