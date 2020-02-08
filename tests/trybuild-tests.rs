#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/macro-exists.rs");
    t.pass("tests/no-pattern-found.rs");
    t.pass("tests/self-position.rs");
    t.pass("tests/types-with-paths.rs");
}
