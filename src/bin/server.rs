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
        }

    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
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
