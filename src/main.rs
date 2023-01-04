use rs_ws281x::{ChannelBuilder, ControllerBuilder};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(21)
                .count(200)
                .strip_type(rs_ws281x::StripType::Ws2811Rgb)
                .build(),
        )
        .build()
        .unwrap();

    println!("Hello, world!");

    let leds = controller.leds_mut(0);

    let sleep_time = 160;

    let color_1: [u8; 4] = [0, 255, 255, 0];
    let color_2: [u8; 4] = [255, 0, 255, 0];

    for i in 0..100 {
        for led in controller.leds_mut(0) {
            if i % 2 == 0 {
                *led = color_1;
            } else {
                *led = color_2;
            }
        }
        controller.render().unwrap();
        sleep(Duration::from_millis(sleep_time));
    }

    println!("Done");
}
