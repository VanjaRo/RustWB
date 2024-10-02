struct Example(i32);

//  This means when an instance of Example goes out of scope or is manually dropped,
// the drop method will be called, and it will print the integer (self.0) contained in that Example instance
impl Drop for Example {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

struct ExampleWrap(Example);

// When an ExampleWrap is dropped:
// 1. The inner Example value is replaced by a new Example(0) using std::mem::replace
// 2. This prevents the original Example from being dropped by the usual mechanism at this point
//    The e variable now holds the original Example that was replaced.
// 3. It prints wrap {value of e.0}
// 4. After the drop method of ExampleWrap completes, Rust will automatically drop the newly replaced Example(0) and call its drop method, printing 0
impl Drop for ExampleWrap {
    fn drop(&mut self) {
        let e = std::mem::replace(&mut self.0, Example(0));
        println!("wrap {}", e.0);
    }
}

fn main() {
    // The instance Example(1) is created but not assigned to a variable
    // It will be immediately dropped at the end of the current statement, printing 1
    Example(1);

    // Which is dropped when it goes out of scope at the end of the main function, printing 2
    let _e2 = Example(2);

    // It will also be dropped at the end of the main function, printing 3
    let _e3 = Example(3);

    // This creates an instance of Example(4) that is immediately dropped after creation because it isn't bound to a variable
    let _ = Example(4);

    // _e5 is initially uninitialized. Later, it is assigned a Some(Example(5)).
    // However, the drop method is not called yet because the value is wrapped in Some, meaning the ownership hasn't been released
    let mut _e5;

    _e5 = Some(Example(5));

    // Now _e5 is reassigned to None. This means Example(5) will be dropped as part of the Some to None transition, and it prints 5
    _e5 = None;

    // The drop(e6) function is called explicitly, immediately dropping Example(6), which prints 6
    let e6 = Example(6);

    drop(e6);

    // std::mem::forget(e7) is called, which tells Rust to "forget" this instance, preventing drop from being called.
    // Therefore, Example(7) is not dropped, and no output is printed for this instance.
    let e7 = Example(7);

    std::mem::forget(e7);

    // When ExampleWrap is dropped at the end of main, its drop method is called.
    // Inside the drop method, std::mem::replace replaces the internal Example(8) with Example(0) and returns the original Example(8) into e.
    // It prints wrap 8.
    // After the drop method of ExampleWrap finishes, Example(0) is dropped, printing 0
    ExampleWrap(Example(8));

    // Scope variables are dropped in reverse order of declaration. Printing 3 and 2.
}
