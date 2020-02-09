# impl-template

`impl-template` is a procedural macro for the Rust programming language that allows you to define templates for "impl"-items and have them expanded to several instances depending on the configuration.

In a way, you can think of it as compile-time blanket impls if you know all types you want to implement it for upfront.

## Usage

```rust
trait Foo {}

struct Bar;
struct Baz;

#[impl_template]
impl Foo for ((Bar, Baz)) {

}
```

This will generate the following code:

```rust
impl Foo for Bar {

}

impl Foo for Baz {

}
```

## Advanced usage


`impl-template` looks for patterns of double tuples.
Those are syntactically valid Rust code but AFAIK fairly useless and should thus not appear in day-to-day Rust-code.

### Several double tuple patterns

You can have as many of those double-tuples as you want.
`impl-template` will create a cartesian product out of all of them and generate the impl-blocks accordingly.

```rust
trait GenericFoo<T, S> { }

struct Bar;
struct Baz;

struct One;
struct Two;
struct Three;

struct Alpha;
struct Beta;

#[impl_template]
impl GenericFoo<((Bar, Baz)), ((Alpha, Beta))> for ((One, Two, Three)) { }
```
The above snippet will expand to 12 impl blocks (2 * 3 * 2).

### Referring to types

`impl-template` allows you to refer to types within the template block.
It generates a dummy identifier for every double-tuple in the scheme of `__TYPE{}__` with `{}` being replaced with a 0-based index.

We can extend `GenericFoo` with a method where we have to name the type parameters:

```rust
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
```

The above code expands to the following (non-exhaustive list):

```rust
impl GenericFoo<Bar, Alpha> for One {
    fn my_fn(_arg1: Bar, _arg2: Alpha) -> One {
        unimplemented!()
    }
}

impl GenericFoo<Bar, Beta> for One {
    fn my_fn(_arg1: Bar, _arg2: Beta) -> One {
        unimplemented!()
    }
}
```

In other words, `__TYPE0__` is an iterator-like placeholder for the first double-tuple `((Bar, Baz))`, `__TYPE1__` for the second one, etc.
