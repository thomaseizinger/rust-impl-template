use impl_template::impl_template;

trait GenericFoo<T> {
    fn do_something(&self) -> T;
}

struct Bar;
struct Baz;

struct Xyz;

#[impl_template]
impl GenericFoo<((Bar, Baz))> for Xyz {
    fn do_something(&self) -> __TYPE__ {
        unimplemented!()
    }
}

fn main() {
    assert_impls_foo::<Xyz, Bar>();
    assert_impls_foo::<Xyz, Baz>();
}

fn assert_impls_foo<F: GenericFoo<T>, T>() {}
