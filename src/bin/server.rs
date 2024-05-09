extern crate notify;

use tonic::{transport::Server, Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, HelloNumber};
use notify::event::{AccessKind, Event};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Config, EventKind};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn say_bye(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a bye request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Bye {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn say_repeat(
        &self,
        request: Request<HelloNumber>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a repeat request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("{}!", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default().with_poll_interval(std::time::Duration::from_secs(2)).with_compare_contents(true)
    )?;

    Ok((watcher, rx))
}

async fn watch_file_write<P: AsRef<std::path::Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {

        if let Some(_event) = res.ok().filter(|event| event.kind == EventKind::Access(AccessKind::Close(notify::event::AccessMode::Write))) {
            println!("Some file done writing!");

            // let files_to_process: Vec<PathBuf> = event.paths.into_iter() 
            //     .filter(|f| {
            //         f.is_file() 
            //         && f.file_name().and_then(OsStr::to_str).is_some_and(|n| !n.starts_with("done"))
            //         && f.extension().is_some_and(|n| n == "parquet")
            //     }).collect();

            // log::info!("{:?}",files_to_process);
            // for filepath in files_to_process.into_iter()
            //     {
            //         let semaphore_clone = semaphore.clone();
            //         let db_pool_clone = db_pool.clone();

            //         let _ = tokio::spawn(async move {
            //             log::info!("Waiting for permit to read newly created file");
            //             let permit = semaphore_clone.acquire().await.unwrap();
            //             log::info!("Successfully acquire permit to read newly created file");

            //             let program_start_time = Instant::now();
            //             match read_defer_record_from_parquet(filepath.clone()) {

            //                 Ok(defer_records) => {

            //                     if let Err(e) = insert_defer_record_into_database(defer_records, &db_pool_clone).await {

            //                         log::error!("{:?}", e.to_string());

            //                     } else {

            //                         log::info!("Done inserting file {:?}", filepath);
            //                         let name = filepath.file_name().unwrap();
            //                         let mut new_name = filepath.clone();
            //                         new_name.set_file_name(format!("done_{}", name.to_str().unwrap()));
            //                         if let Some(ext) = filepath.extension() {

            //                             new_name.set_extension(ext);
            //                         }

            //                         if let Err(e) = rename(filepath, new_name) {
            //                             log::error!("Error occured when renaming file! {}", e.to_string());
            //                         }
            //                     }
            //                 },
            //                 Err(err) => {
            //                     log::error!("Some Error occur while reading from parquet file: {}", err.to_string());
            //                 }
            //             };
            //             log::info!("raw data import finished in {:.2?}, waiting for new task", program_start_time.elapsed());
            //             drop(permit);
            //         }).await;

            //     }
        }

    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    tokio::spawn(watch_file_write("/home/chunhou/Dev/rust/file-tagger/test/"));
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
