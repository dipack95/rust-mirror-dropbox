//extern crate notify;
//extern crate walkdir;
//
//use notify::{RecommendedWatcher, Watcher, RecursiveMode};
//use walkdir::WalkDir;
//
//fn watch(watch_directory: String) -> notify::Result<()> {
//    let (tx, rx) = std::sync::mpsc::channel();
//
//    let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(2))?;
//
//    watcher.watch(watch_directory, RecursiveMode::Recursive)?;
//
//    loop {
//        match rx.recv() {
//            Ok(event) => println!("Something happened: {:?}", event),
//            Err(event) => println!("Watch error: {:?}", event)
//        }
//    }
//}
//
//fn get_accessible_sub_directories(directory: String) {
//    for dir in WalkDir::new(directory).into_iter().filter_map(|d| d.ok()) {
//        println!("{}", dir.path().display());
//    }
//}
//
//fn main() {
////    let watch_directory: String = "D:\\Scripts".to_string();
////
////    if let Err(event) = watch(watch_directory) {
////        println!("Main thread: Error: {:?}", event)
////    }
//
//    let hard_reference = &4.15;
//    println!("{:?}", hard_reference);
//
//    let ref another_hard_reference= 4.36;
//    println!("{:?}", another_hard_reference);
//    println!("{:?}", *another_hard_reference);
//
//    let mut mutable_value = 10;
//
//    match mutable_value {
//        ref mut m => {
//            let ref mut local_mut_ref = m.clone();
//            *local_mut_ref += 1;
//            println!("{:?}", *local_mut_ref);
//        }
//    }
//
//    println!("{:?}", mutable_value);
//}

fn is_divisible_by(numerator: i32, denominator: i32) -> bool {
    return numerator % denominator == 0;
}

fn fizzbuzz(num: u32) {
    if is_divisible_by(num as i32, 15 as i32) {
        println!("FizzBuzz");
    } else if is_divisible_by(num as i32, 5 as i32) {
        println!("Fizz");
    } else if is_divisible_by(num as i32, 3 as i32) {
        println!("Buzz");
    }
}

fn fizzbuzz_until(num: u32) {
    for n in 0..num {
        fizzbuzz(n);
    }
}

fn main() {
    let max_fizzbuzz: u32 = 100;
    fizzbuzz_until(max_fizzbuzz);
}