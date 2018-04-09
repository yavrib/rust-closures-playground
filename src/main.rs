fn main() {
    // Err since our function reference does not have a lifetime specified
    fn no_lifetime_specified_higher_order_function() -> &(Fn(i32) -> i32) {
       let num = 5;

        |x| x + num
    }

    // Err because this returns closure environment, not just a function
    fn static_lifetime_specified_higher_order_function() -> &'static (Fn(i32) -> i32) {
        let num = 5;

        |x| x + num
    }

    // Err because we still need to move num into the boxed function
    fn boxed_higher_order_function() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(|x| x + num)
    }

    fn higher_order_function() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }

    let f = higher_order_function();
    let result = f(5);

    println!("{}", result)
}
