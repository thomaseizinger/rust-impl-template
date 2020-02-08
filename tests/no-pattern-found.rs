use impl_template::impl_template;

trait Foo {

}

struct Bar;

#[impl_template]
impl Foo for Bar {

}

fn main() {
    assert_impls_foo::<Bar>();
}

fn assert_impls_foo<F: Foo>() {

}
