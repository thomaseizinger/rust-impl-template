use impl_template::impl_template;

trait Foo {}

struct Bar;
struct Baz;

#[impl_template]
impl Foo for ((Bar, Baz)) {}

fn main() {
    assert_impls_foo::<Bar>();
    assert_impls_foo::<Baz>();
}

fn assert_impls_foo<F: Foo>() {}
