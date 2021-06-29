#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};

mod paste_id;
use paste_id::PasteId;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(12);
    let filename = format!("upload/{}", id);
    paste.open(128.kibibytes()).into_file(filename).await?;

    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);
    Ok(format!("{url}", url = url))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload])
}
