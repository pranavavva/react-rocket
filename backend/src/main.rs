#[macro_use]
extern crate rocket;
extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    text: &'static str,
}

#[get("/")]
fn index() -> String {
    let message = Message {
        text: "this is an edit!",
    };

    serde_json::to_string(&message).unwrap()
}

fn rocket() -> _ {
    rocket::ignite().mount("/api", routes![index])
}
