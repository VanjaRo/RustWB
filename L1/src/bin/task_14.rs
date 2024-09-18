use std::any::Any;

// knows type T at compile time, not suitable
fn print_type_of<T: Any>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}

fn main() {
    let a: i32 = 42;
    let b: &str = "Hello";
    let c: f64 = 3.14;

    print_type_of(&a);
    print_type_of(&b);
    print_type_of(&c);

    let values: Vec<Box<dyn Any>> = vec![Box::new(a), Box::new(b), Box::new(c)];

    // Using downcast_ref to get the value of dyn Any type
    // Before casting we ensure that the type we are casting is the real type of the value
    for value in values.iter() {
        if value.is::<i32>() {
            println!(
                "Found an i32 with value: {}",
                value.downcast_ref::<i32>().unwrap()
            );
        } else if value.is::<&str>() {
            println!(
                "Found a &str with value: {}",
                value.downcast_ref::<&str>().unwrap()
            );
        } else if value.is::<f64>() {
            println!(
                "Found a f64 with value: {}",
                value.downcast_ref::<f64>().unwrap()
            );
        } else {
            println!("Unknown type");
        }
    }
}
