/// This module shows some KEY concepts of Rust related to
///     functional programming
///     closure
///     iterators
/// See
///     https://doc.rust-lang.org/book/ch13-00-functional-features.html
///
//Rust’s design has taken inspiration from many existing languages and techniques,
// and one significant influence is functional programming.
// Programming in a functional style often includes
//      using functions as values by passing them in arguments,
//      returning them from other functions,
//      assigning them to variables for later execution,
// and so forth.
//
// In this section we'll discuss some features of Rust
// that are similar to features in many languages often referred to as functional.
// Two important features include
//      closures
//      iterators

/* ======= Closures ========
   ========================= */
pub mod closures{

    // Rust’s closures are anonymous functions you can save in a variable
    // or pass as arguments to other functions.
    // You can create the closure in one place
    // and then call the closure elsewhere to evaluate it in a different context.
    // Unlike functions, closures can capture values from the scope in which they’re defined.
    // We’ll demonstrate how these closure features allow for code reuse and behavior customization.

    // A function that increments by 1
    fn function(i: i32) -> i32 { i + 1 }

    pub fn closuresexample() {
        // Closures are anonymous, here we are binding them to references
        // Annotation is identical to function annotation but is optional
        // as are the `{}` wrapping the body. These nameless functions
        // are assigned to appropriately named variables.
        let closure_annotated = |i: i32| -> i32 { i + 1 };
        // between | and | is the formal parameter
        let closure_inferred = |i| i + 1;
        // type annotations are optional, as are curly braces
        //      don't be fools and use them

        let i = 1;
        // Call the function and closures.
        println!("function: {}", function(i));
        println!("closure_annotated: {}", closure_annotated(i));
        println!("closure_inferred: {:?}", closure_inferred(i));

        // A closure taking no arguments which returns an `i32`.
        // The return type is inferred.
        let one = || -> i32 { 1 };
        println!("closure returning one: {}", one());

        // Closures they are usually short and relevant only within a narrow context
        // rather than in any arbitrary scenario.
        // Within these limited contexts, the compiler is reliably able to infer the types
        // of the parameters and the return type, similar to how it’s able to infer the types of most variables.

        // Closures (and functions) can be used as inputs and as output to other functions
        // this makes the language:
        //      higher-order
        let closure = |x : i32| -> i32 { println!("I'm a closure with {}! ", x);x };

        call_me(closure);
        call_me(function);
    }

    fn call_me<F: Fn(i32) -> i32>(f: F) {
        f(1);
    }

    // Closures as input parameters are possible,
    // so returning closures as output parameters should also be possible.
    // Before we deal with them, we need to explain capturing, and the related traits

    // capturing is not a necessary feature of closures
    // you can write closures that do not capture anything, and that's still very FP
    // but when capturing, things get real messy, real quick

    //Closures are inherently flexible and will do what the functionality requires
    // to make the closure work without annotation.
    // This allows capturing to flexibly adapt to the use case,
    // sometimes moving and sometimes borrowing.
    // Closures can capture variables:
    //      by reference: &T
    //      by mutable reference: &mut T
    //      by value: T
    // They preferentially capture variables by reference and only go lower when required.

    pub fn capturingexample(){
        use std::mem;

        let color = String::from("green");
        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last time.
        //
        // `println!` only requires arguments by immutable reference so it doesn't
        // impose anything more restrictive.
        // LOOK CLOSELY! the body of the closure is referring to color!
        let print = || { println!("`color`: {}", color) };

        // Call the closure using the borrow.
        print();

        // QUIZ: can we do this:
        // Y | N
        let _reborrow = &color;
        print();
        // `color` can be borrowed immutably again, because the closure only holds
        // an immutable reference to `color`.

        // A move or reborrow is allowed after the final use of `print`
        let _color_moved = color;

        let mut count = 0;
        // A closure to increment `count` could take either `&mut count` or `count`
        // but `&mut count` is less restrictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` because a `&mut` is stored inside. Thus,
        // calling the closure mutates the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called later.
        // QUIZ: can we do this, given the inc() below?
        // let _reborrow = &count;
        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // possible to reborrow without an error
        let _count_reborrowed = &mut count;

        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move and so `movable` immediately moves into
        // the closure.
        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable);
        };

        // `consume` consumes the variable so this can only be called once.
        consume();
        // DNC: error[E0382]: use of moved value: `consume`
        // consume();
        // The error is interesting:
        // 153 |         consume();
        //     |         ^^^^^^^
        //      note: this value implements `FnOnce`, which causes it to be moved when called
    }

    // While Rust chooses how to capture variables on the fly mostly without type annotation,
    // this ambiguity is not allowed when writing functions.
    // When taking a closure as an input parameter,
    // the closure's complete type must be annotated using one of a few traits,
    // and they're determined by what the closure does with captured value.
    // In order of decreasing restriction, they are:
    //      Fn: the closure uses the captured value by reference (&T)
    //      FnMut: the closure uses the captured value by mutable reference (&mut T)
    //      FnOnce: the closure uses the captured value by value (T)
    // On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.
    //
    // For instance, consider a parameter annotated as FnOnce.
    // This specifies that the closure may capture by &T, &mut T, or T,
    // but the compiler will ultimately choose
    // based on how the captured variables are used in the closure.
    //
    // This is because if a move is possible,
    // then any type of borrow should also be possible.
    // Note that the reverse is not true.
    // If the parameter is annotated as Fn, then capturing variables by &mut T or T are not allowed.

    // let's look at an example

    // A function which takes a closure as an argument and calls it.
    // <F> denotes that F is a "Generic type parameter"
    fn apply_FnOnce<F>(f: F) where F: FnOnce() {
        // Note: The closure takes no input and returns nothing.
        f();
    }
    fn apply_Fn<F>(f: F) where F: Fn() {
        // Note: The type of F has changed
        f();
    }
    fn apply_FnMut<F>(mut f: F) where F: FnMut() {
        // Note: The type of F has changed and we needed to add the `mut` to F
        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        // Note: The closure takes an `i32` and returns an `i32`.
        f(3)
    }

    pub fn fntypes(){
        use std::mem;

        let greeting = "hello";
        // A non-copy type.
        // `to_owned` creates owned data from borrowed one
        let mut farewell = "goodbye".to_owned();

        // Capture 2 variables: `greeting` by reference and
        // `farewell` by value.
        let diary = || {
            // `greeting` is by reference: requires `Fn`.
            println!("I said {}.", greeting);

            // Mutation forces `farewell` to be captured by
            // mutable reference. Now requires `FnMut`.
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            // Manually calling drop forces `farewell` to
            // be captured by value. Now requires `FnOnce`.
            mem::drop(farewell);
        };

        // Call the function which applies the closure.
        apply_FnOnce(diary);
        // DNC: error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
        // QUIZ: what do i need to comment in the code of `diary` to make this work
        // apply_FnMut(diary);
        // if we comment the `mem::drop``, and the previous usage, this works
        // DNC: error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        // QUIZ: what do i need to comment in the code of `diary` to make this work
        // apply_Fn(diary);
        // we need to comment both the `mem::drop` and the `farewell.push` to make this go
        // and now we can call both apply_FnOnce and apply_FnMut too

        // `double` satisfies `apply_to_3`'s trait bound
        let double = |x| 2 * x;

        println!("3 doubled: {}", apply_to_3(double));
    }

    // Back to Returning closures
    // Anonymous closure types are, by definition,
    // unknown, so we have to use impl Trait to return them.
    //
    // The valid traits for returning a closure are
    //      Fn
    //      FnMut
    //      FnOnce
    // Beyond this, the
    //      move
    // keyword must be used,
    // which signals that all captures occur by value.
    // This is required because any captures by reference would be dropped
    // as soon as the function exited, leaving invalid references in the closure

    // we define 3 functions that return closures
    // see the impl in the return type
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    pub fn closures_output(){
        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();
    }

    // now a big FP example that uses closures

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    // closures are used a lot in Options and Iterators
    pub fn fprules() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;
        // TODO: what's 0.. ?

        // Imperative approach
        // Declare accumulator variable
        let mut acc = 0;
        // Iterate: 0, 1, 2, ... to infinity
        for n in 0.. {
            // Square the number
            let n_squared = n * n;

            if n_squared >= upper {
                // Break loop if exceeded the upper limit
                break;
            } else if is_odd(n_squared) {
                // Accumulate value, if it's odd
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);

        // Functional approach
        let sum_of_squared_odd_numbers: u32 =
            (0..).map(|n| n * n)                             // All natural numbers squared
                .take_while(|&n_squared| n_squared < upper) // Below upper limit
                .filter(|&n_squared| is_odd(n_squared))     // That are odd
                .fold(0, |acc, n_squared| acc + n_squared); // Sum them
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }

}

/* ======= Iterators =======
   ========================= */

pub mod iterators{
    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // An iterator is responsible for the logic of iterating over each item
    // and determining when the sequence has finished.
    // When you use iterators, you don’t have to reimplement that logic yourself.

    // In Rust, iterators are lazy,
    // meaning they have no effect until you call methods that consume the iterator to use it up.
    pub fn iteratorexample(){
        // For example, the code in the example below creates an iterator
        // over the items in the vector v1 by calling
        // the iter method defined on Vec\<T\>.
        // This code by itself doesn’t do anything useful.
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        // These iterators can be used in a variety of ways.
        // For example, we can separate the creation of the iterator
        // from the use of the iterator in the for loop, as shown below.
        for val in v1_iter {
            println!("Got: {}", val);
        }
        //Other languages that don’t have iterators provided by their standard libraries,
        // you would likely write this same functionality by starting a variable at index 0,
        // using that variable to index into the vector to get a value,
        // and incrementing the variable value in a loop until it reached the total number of items in the vector.
        //
        // Iterators handle all that logic for you,
        // cutting down on repetitive code you could potentially mess up.
        // Iterators give you more flexibility to use the same logic
        // with many different kinds of sequences,
        // not just data structures you can index into, like vectors.
        //
        // Let’s examine how iterators do that.

        // All iterators implement a trait named Iterator
        // that is defined in the standard library.
        // The definition of the trait looks like this:

        // pub trait Iterator {
        //     type Item;
        //
        //     fn next(&mut self) -> Option<Self::Item>;
        //     // methods with default implementations elided
        // }

        // Notice this definition uses some new syntax:
        //      type Item and Self::Item,
        // which are defining an associated type with this trait.
        // For now, all you need to know is that this code says
        // implementing the Iterator trait
        // requires that you also define an Item type,
        // and this Item type is used in the return type of the next method.
        // In other words, the Item type will be the type returned from the iterator.
        //
        // The Iterator trait only requires implementors to define one method:
        // the next method, which returns one item of the iterator at a time
        // wrapped in Some and, when iteration is over, returns None.

        let v1 = vec![1, 2, 3];

        // Note that we need to make v1_iter mutable:
        // calling the next method on an iterator changes internal state
        // that the iterator uses to keep track of where it is in the sequence.
        // In other words, this code consumes, or uses up, the iterator.
        // Each call to next eats up an item from the iterator.
        // We didn’t need to make v1_iter mutable when we used a for loop
        // because the loop took ownership of v1_iter and made it mutable behind the scenes.
        let mut v1_iter = v1.iter();
        // QUIZ: what does v1_iter.next() return?

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // Also note that the values we get from the calls to next are immutable references
        // to the values in the vector.
        // The iter method produces an iterator over immutable references.
        // If we want to create an iterator that takes ownership of `v1`
        // and returns owned values, we can call into_iter instead of iter.
        // Similarly, if we want to iterate over mutable references,
        // we can call iter_mut instead of iter.

        // Methods can be written to consume the iterator,
        // and these that call next are called consuming adaptors,
        // because calling them uses up the iterator.

        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // One example is the sum method, which takes ownership of the iterator
        // and iterates through the items by repeatedly calling next,
        // thus consuming the iterator.
        // As it iterates through, it adds each item to a running total
        // and returns the total when iteration is complete.
        let total: i32 = v1_iter.sum();

        // DNC: error[E0596]: cannot borrow `v1_iter` as mutable, as it is not declared as mutable
        // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
        // let v2 = v1_iter.next();

        assert_eq!(total, 6);

        // Another kind of method is known as an iterator adaptor,
        // which can allow you to change iterators into different kinds of iterators.
        // You can chain multiple calls to iterator adaptors to perform complex actions
        // in a readable way.
        // But because all iterators are lazy,
        // you have to call one of the consuming adaptor methods
        // to get results from calls to iterator adaptors, as shown below.

        // However, the following code doesn't do anything since
        // the closure we specified never gets called.
        // The reason for this is that iterator adaptors are lazy,
        // and will only consume the iterator when needed.
        let v1: Vec<i32> = vec![1, 2, 3];
        v1.iter().map(|x| x + 1);

        // We can finish this example as shown below.

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }


    // We can now demonstrate a common use of closures
    // that capture their environment by using the filter iterator adaptor.
    // The filter method on an iterator takes a closure
    // that takes each item from the iterator and returns a Boolean.
    // If the closure returns true,
    //      the value will be included in the iterator produced by filter.
    // If the closure returns false,
    //      the value won’t be included in the resulting iterator.

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    // In the following example, we use filter with a closure that captures
    // the shoe_size variable from its environment
    // to iterate over a collection of Shoe struct instances.
    // It will return only shoes that are the specified size.
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    pub fn filters_by_size() {
        // create 3 shoes in a vec
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        // call the filter with param 10
        let in_my_size = shoes_in_size(shoes, 10);
        println!("filtered vec: {:?}", in_my_size );
    }



    // There are other methods implemented on iterators within rust
    // that are similar to one used in functional programming langauges.
    // Some examples of these include
    //      map,
    //      fold,
    //      sum,
    //      filter,
    //      zip, etc.
    //
    // The following example uses map, which we've seen before.
    // The map() method applies a function to each element in an iterable
    // and returns the resulting iterable, of each iteration, to the next function.
    //
    pub fn examplefpiterators() {
        let vector = [1, 2, 3];
        let result = vector.iter().map(|x| x * 2).collect::<Vec<i32>>();
        // another way to write the above statement
        /*let result: Vec<i32> = vector.iter().map(|x| x * 2).collect();*/
        println!("After mapping: {:?}", result);

        // The following example uses sum,
        // which takes an iterator and generates Self from the elements by “summing up” the items.
        //
        let su: u32 = vec![1, 2, 3, 4, 5, 6].iter().sum();
        //
        // The following example accomplishes the same thing using fold. Folding is useful whenever you have a collection of something, and want to produce a single value from it.
        //
        let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().fold(0, |mut summ, &val| {
            summ += val;
            summ
        });
        //
        // The following example uses collect, which consumes the iterator and collects the resulting values into a collection data type.
        //
        let a = [1, 2, 3];
        let doubled: Vec<i32> = a.iter()
            .map(|&x| x * 2)
            .collect();

        assert_eq!(vec![2, 4, 6], doubled);
        //
        // A longer list of these functions can be found [here](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
    }

    // Previously we've created iterators by calling iter, into_iter, or iter_mut on a vector.
    // You can create iterators from the other collection types
    // in the standard library, such as hash map.
    // You can also create iterators that do anything you want
    // by implementing the Iterator trait on your own types.
    // As previously mentioned, the only method you’re required to provide
    // a definition for is the next method.
    //
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    // We can implement an iterator for this struct as shown below.
    impl Iterator for Counter {
        // we need to define what the Item type of the iterator is
        type Item = u32;

        // and next returns an option of that item
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    // We can use the iterator as shown below.
    pub fn calling_next_directly() {
        let mut counter = Counter::new();

        // QUIZ: what will counter.next be?
        // 0 | 1 | Some(1) | Some(0) | None

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    //
    // Note, that with the simple implementation of this next method,
    // we can use various other methods associated with the iterator trait.
    //
    pub fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))// TODO explain zip
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        // QUIZ: what does sum contain?
        assert_eq!(18, sum);
    }

}