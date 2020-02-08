use impl_template::impl_template;

trait Foo {}

mod types {
    pub struct Bar;
    pub struct Baz;
}

#[impl_template]
impl Foo for ((types::Bar, types::Baz)) {}

fn main() {
    assert_impls_foo::<types::Bar>();
    assert_impls_foo::<types::Baz>();
}

fn assert_impls_foo<F: Foo>() {}
