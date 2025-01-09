#[repr(C)]
#[derive(Debug)]
struct Foo {
    bar: u32,
}

#[no_mangle]
extern "C" fn print_foo(foo: *const Foo) {
    println!("RUST | foo: {:?}", foo);
}
