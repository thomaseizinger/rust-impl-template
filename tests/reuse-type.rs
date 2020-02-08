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
        __TYPE__
    }
}

fn main() {
    assert_impls_foo::<Xyz, Bar>();
    assert_impls_foo::<Xyz, Baz>();

    let _bar: Bar = Xyz.do_something();
    let _baz: Baz = Xyz.do_something();
}

fn assert_impls_foo<F: GenericFoo<T>, T>() {}
