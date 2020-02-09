#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro-exists.rs");
    t.pass("tests/no-pattern-found.rs");
    t.pass("tests/self-position.rs");
    t.pass("tests/types-with-paths.rs");
    t.pass("tests/use-in-generics.rs");
    t.pass("tests/reuse-type.rs");
    t.pass("tests/two-patterns.rs");
    t.pass("tests/three-patterns.rs");
}
