use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use redditplace::pixel_event::{PixelEvent,Pixel};
use std::io;
use png;

// PNG saving related tasks
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

// who needs 12 factor app methodology out here... monkas
static KAFKA_URL: &'static str = "localhost:9092";
static EVENT_TOPIC_NAME: &'static str = "pixel-events";
static NUM_PIXELS_WIDE: u32 = 1000;
static NUM_PIXELS_HIGH: u32 = 1000;
static NUM_PIXELS: usize = 1000000;

fn handle_pixel_event(canvas: &[u8], ) {
}

fn save_canvas(canvas: &Vec<Pixel>) {
    let path = Path::new(r"canvas.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    // Write PNG image to the above file
    let mut encoder = png::Encoder::new(w, NUM_PIXELS_WIDE, NUM_PIXELS_HIGH);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(
        &canvas.iter()
            .flat_map(|p| p.as_bytes())
            .collect::<Vec<u8>>()
            .as_slice()
    ).unwrap();
}

fn main() -> Result<(), io::Error> {
    let mut consumer = Consumer::from_hosts(vec!(String::from(KAFKA_URL)))
        .with_topic(String::from(EVENT_TOPIC_NAME))
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    let canvas: Vec<Pixel> = vec![Pixel::new(255,0,0,255); NUM_PIXELS];
    save_canvas(&canvas);

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", m);
            }
            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}