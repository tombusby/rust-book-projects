#[macro_use]
extern crate rocket;
use rocket::tokio::task::spawn_blocking; // This converts sync op to async
use rocket::tokio::time::{sleep, Duration};
use std::{fs, io};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    println!("About to sleep!");
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| fs::read("data/foo.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes!(index, delay, blocking_task))
}
