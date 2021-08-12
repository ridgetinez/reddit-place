use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use kafka::producer::{Producer, Record, RequiredAcks};
use redditplace::pixel_event::{PixelEvent, Pixel};
use std::time::Duration;
use std::io;

// who needs 12 factor app methodology out here... monkas
static KAFKA_URL: &'static str = "localhost:9092";
static EVENT_TOPIC_NAME: &'static str = "pixel-events";

#[post("/draw")]
async fn handle_draw() -> impl Responder {
    let pixel_event = PixelEvent{
        x: 2502,
        y: 256,
        pixel: Pixel{
            r: 250,
            g: 250,
            b: 250,
            a: 250,
        },
    };
    let serialised = serde_json::to_string(&pixel_event).unwrap();

    // Each handler defines its own producer client, as it can't be shared in an Arc
    // context without a Mutex / guarantee of no race conditions. What happens to performance?
    let mut producer = Producer::from_hosts(vec![String::from(KAFKA_URL)])
                    .with_ack_timeout(Duration::from_secs(1))
                    .with_required_acks(RequiredAcks::One)
                    .create()
                    .unwrap(); // #yolo
    producer.send(&Record::from_value(EVENT_TOPIC_NAME, serialised)).unwrap();
    HttpResponse::Ok().body(String::from("some random shit"))
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    println!("Starting ingestion server...");
    HttpServer::new(|| {
        App::new()
            .service(handle_draw)
    })
    .bind("localhost:8081")?
    .run()
    .await
}
