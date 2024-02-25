extern crate notify;

use notify::event::{CreateKind, RemoveKind};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Config, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

fn main() {
    // Create a channel to communicate with the watcher thread
    let (tx, rx) = channel();

    let config = Config::default().with_poll_interval(Duration::from_secs(2)).with_compare_contents(true);
    // Define the directory to watch
    // Create a watcher object, handling errors if any
    let mut watcher: RecommendedWatcher = match Watcher::new(tx, config){
        Ok(w) => w,
        Err(e) => panic!("Failed to initialize watcher: {:?}", e),
    };

    let path = Path::new("/home/chunhou/Dev/rust/file-tagger/test");

    // Watch the directory for changes
    if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
        println!("Failed to watch directory: {:?}", e);
        return;
    }

    println!("Watching directory for changes...");

    // Start an infinite loop to receive events from the watcher thread
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {

                    Ok(e) => {

                        match e.kind {

                            EventKind::Create(CreateKind::File) => {
                                println!("file {:?} created", e.paths);
                            },

                            EventKind::Remove(RemoveKind::File) => {
                                println!("Some file removed");
                            }
                            _ => {
                                println!("some other even occur");
                            }
                        }
                    },
                    Err(err) => {
                        println!("Some error occur {:?}", err);
                    }

                }
                // Handle the file change event here
            },
            Err(e) => println!("Watcher error: {:?}", e),
        }
    }
}
