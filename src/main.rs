extern crate notify;
extern crate walkdir;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use walkdir::WalkDir;
use std::collections::LinkedList;

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

fn get_accessible_objects(directory: &str, max_depth: Option<u32>) -> LinkedList<String> {
    let mut accessible_entries = LinkedList::new();
    for dir in WalkDir::new(directory).max_depth(max_depth.unwrap_or(1) as usize).into_iter().filter_map(|d| d.ok()) {
        accessible_entries.push_back(dir.path().display().to_string());
    }
    accessible_entries
}

fn main() {
    let root_directory: &str = "D:\\";
    let watch_directory: &str = "D:\\Scripts";

    let entries: LinkedList<String> = get_accessible_objects(root_directory, Some(2));

    println!("Number of all accessible objects under {} is {:?}", root_directory, entries.len());

    let mut iter = entries.iter();
    loop {
        match iter.next() {
            Some(ob) => { println!("Object: {:?}", ob) }
            _ => {
                println!("Nothing found!");
                break;
            }
        }
    }

    if let Err(event) = watch(watch_directory) {
        println!("Main thread: Error: {:?}", event)
    }
}