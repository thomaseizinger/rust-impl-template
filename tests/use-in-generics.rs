use impl_template::impl_template;

trait GenericFoo<T> {}

struct Bar;
struct Baz;

struct Xyz;

#[impl_template]
impl GenericFoo<((Bar, Baz))> for Xyz {}

fn main() {
    assert_impls_foo::<Xyz, Bar>();
    assert_impls_foo::<Xyz, Baz>();
}

fn assert_impls_foo<F: GenericFoo<T>, T>() {}
