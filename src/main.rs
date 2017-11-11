extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};

fn watch(watch_directory: String) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(5))?;

    watcher.watch(watch_directory, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => println!("Something happened: {:?}", event),
            Err(event) => println!("Watch error: {:?}", event)
        }
    }
}

fn main() {
    let watch_directory: String = "D:\\Scripts".to_string();
    if let Err(event) = watch(watch_directory) {
        println!("Main thread: Error: {:?}", event)
    }
}