extern crate notify;
extern crate walkdir;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use walkdir::WalkDir;

fn watch(watch_directory: &str) -> notify::Result<()> {
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

fn get_accessible_sub_directories(directory: &str) {
    for dir in WalkDir::new(directory).into_iter().filter_map(|d| d.ok()) {
        println!("{}", dir.path().display());
    }
}

fn main() {
    let watch_directory = "D:\\Scripts";

//    get_accessible_sub_directories(watch_directory);

    if let Err(event) = watch(watch_directory) {
        println!("Main thread: Error: {:?}", event)
    }
}