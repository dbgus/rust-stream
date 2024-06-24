extern crate rocket;

#[macro_use]
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use mp4::{Bytes, Result};
use std::str;
use rocket::futures::stream::Stream;
use rocket::response::stream::{stream, Event, EventStream};
use rocket::{get, launch, routes};

use rocket::tokio::time::{self, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/byte/stream")]
fn stream() -> EventStream![] {
    EventStream! {
        let f = File::open("video/videoplayback.mp4").unwrap();
        let size = f.metadata().unwrap().len();
        let reader = BufReader::new(f);
        let mut mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();
        let mut keys = mp4.tracks().keys().copied().collect::<Vec<u32>>();

        loop {
            let mut inx = 0;
            loop {
                if inx == mp4.sample_count(keys.get(0).unwrap().clone()).unwrap() {
                     break;
                }
                inx +=1;
                let sample = mp4.read_sample(keys.get(0).unwrap().clone(), inx);
                let samp = sample.unwrap();
                yield Event::data(format!("{:?}", samp.unwrap().bytes))

            }

            keys.pop();

            if keys.len() == 0 {
                break;

            }
        }

    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, stream])
}
