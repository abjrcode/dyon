fn foo() -> {
    return err("something wrong happened")
}

fn bar() -> {
    x := foo()
    x := x?
    return ok(x + 1)
}

fn baz() -> {
    x := bar()?
    return ok(x + 1)
}

fn main() {
    println(unwrap(baz()))
}
