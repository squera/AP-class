///
/// https://www.newline.co/@kkostov/the-rust-map-function-a-gateway-to-iterators--f991b22b

// do you recall map from functional programming?
// type of map:  list a -> (a->b) -> list b
//
// in Rust, map works on iterators
//      because they're a general concept and all collections implement them
//      so it's easy to lift the notion from lists to hashmaps etc
pub fn singlemap(){
    let numbers = vec![3, 6, 9, 12];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n * 10)
        .collect();
    println!("{:?}",result);
}
// what does map return then? another iterator!
// because we can chain maps!
pub fn twomaps(){
    let numbers = vec![3, 6, 9, 12];
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| n * 10)
        .map(|n| n / 3)
        .collect();
    println!("{:?}",result);
}
// notice the `.collect()`
// it's there to convert the returned iterator into a Vec


// map is LAZY
pub fn lazymap_collect(){
    let numbers = vec![3, 6, 9, 12];
    let mut number_of_times = 0;
    let result: Vec<i32> = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n * 10
        })
        .collect();
    println!("{:?}",result);
    println!("{}",number_of_times);
}
pub fn lazymap_nocollect(){
    let numbers = vec![3, 6, 9, 12];
    let mut number_of_times = 0;
    // notice the change in type here
    let result = numbers
        .iter()
        .map(|n| {
            number_of_times += 1;
            return n * 10
        });
    println!("{:?}",result);
    println!("{}",number_of_times);
}
// QUIZ: what will this print?

pub fn string_tolower(){
    let words: Vec<&str> = vec!["Hello", "from", "Rust", "!"];
    println!("Words before map: {:?}", words);
    let lowercased_words: Vec<String> = words // using String as the return type of `to_lowercase`
        .iter()
        .map(|word| word.to_lowercase())
        .collect();
    println!("Words after map: {:?}", lowercased_words);
}

pub fn maps_options(){
    let str_numbers: Vec<&str> = vec!["1", "2", "3", "I am not a number"];
    let numbers: Vec<u32> = str_numbers
        .iter()
        // does not compile
        // .map(|str_number| str_number.parse::<u32>())
        // must use `flat_map`
        //  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
        .flat_map(|str_number| str_number.parse::<u32>())
        .collect();
    println!("{:?}",numbers);
}