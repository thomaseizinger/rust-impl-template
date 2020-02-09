use impl_template::impl_template;

trait GenericFoo<T> {}

struct Bar;
struct Baz;

struct One;
struct Two;
struct Three;

#[impl_template]
impl GenericFoo<((Bar, Baz))> for ((One, Two, Three)) {}

fn main() {
    assert_impls_foo::<One, Bar>();
    assert_impls_foo::<Two, Bar>();
    assert_impls_foo::<Three, Bar>();

    assert_impls_foo::<One, Baz>();
    assert_impls_foo::<Two, Baz>();
    assert_impls_foo::<Three, Baz>();
}

fn assert_impls_foo<F: GenericFoo<T>, T>() {}
