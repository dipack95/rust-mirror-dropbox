extern crate notify;
extern crate walkdir;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use walkdir::WalkDir;

fn watch(watch_directory: String) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(2))?;

    watcher.watch(watch_directory, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("Something happened: {:?}", event),
            Err(event) => println!("Watch error: {:?}", event)
        }
    }
}

fn get_accessible_sub_directories(directory: String) {
    for dir in WalkDir::new(directory).into_iter().filter_map(|d| d.ok()) {
        println!("{}", dir.path().display());
    }
}

fn main() {
//    let watch_directory: String = "D:\\Scripts".to_string();
//
//    if let Err(event) = watch(watch_directory) {
//        println!("Main thread: Error: {:?}", event)
//    }

    let hard_reference = &4.15;
    println!("{:?}", hard_reference);

    let ref another_hard_reference= 4.36;
    println!("{:?}", another_hard_reference);
    println!("{:?}", *another_hard_reference);

    let mut mutable_value = 10;

    match mutable_value {
        ref mut m => {
            let ref mut local_mut_ref = m.clone();
            *local_mut_ref += 1;
            println!("{:?}", *local_mut_ref);
        }
    }

    println!("{:?}", mutable_value);
}