use impl_template::impl_template;

trait GenericFoo<T, S> {
    fn my_fn(arg1: T, arg2: S) -> Self;
}

struct Bar;
struct Baz;

struct One;
struct Two;
struct Three;

struct Alpha;
struct Beta;

#[impl_template]
impl GenericFoo<((Bar, Baz)), ((Alpha, Beta))> for ((One, Two, Three)) {
    fn my_fn(_arg1: __TYPE0__, _arg2: __TYPE1__) -> __TYPE2__ {
        unimplemented!()
    }
}

fn main() {
    assert_impls_foo::<One, Bar, Alpha>();
    assert_impls_foo::<Two, Bar, Alpha>();
    assert_impls_foo::<Three, Bar, Alpha>();

    assert_impls_foo::<One, Baz, Alpha>();
    assert_impls_foo::<Two, Baz, Alpha>();
    assert_impls_foo::<Three, Baz, Alpha>();

    assert_impls_foo::<One, Bar, Beta>();
    assert_impls_foo::<Two, Bar, Beta>();
    assert_impls_foo::<Three, Bar, Beta>();

    assert_impls_foo::<One, Baz, Beta>();
    assert_impls_foo::<Two, Baz, Beta>();
    assert_impls_foo::<Three, Baz, Beta>();
}

fn assert_impls_foo<F: GenericFoo<T, S>, T, S>() {}
