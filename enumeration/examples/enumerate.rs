use pico_download::cache_resolution;
use pico_enumeration::*;
use pico_streaming::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        // .pretty()
        .init();

    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let (devices, _errors) = enumerator.enumerate().devices_and_errors();

    let device = devices.first().unwrap().open()?;
    println!("{:#?}", device);

    let info = device.device_info()?;

    println!("Info: {:#?}", info);

    let mut config = device.device_config()?.unwrap();

    config.set("samples_per_second", 100)?;

    let channel_a = config.channel("A")?;
    channel_a.set("enabled", true)?;
    channel_a.set("range", "5 V")?;
    channel_a.set("coupling", "DC")?;

    println!("Config: {:#?}", config);

    let device = device.into_streaming_device();

    struct StdoutHandler;

    impl EventHandler for StdoutHandler {
        fn handle_event(&self, event: &StreamingEvent) {
            match event {
                StreamingEvent::Open => {
                    println!("Open");
                }
                StreamingEvent::Close(pico_error) => {
                    println!("Close: {:?}", pico_error);
                }
                StreamingEvent::StreamStart => {
                    println!("StreamStart");
                }
                StreamingEvent::StreamStop => {
                    println!("StreamStop");
                }
                StreamingEvent::StreamData(streaming_result) => {
                    for (name, data) in streaming_result.channels.iter() {
                        println!("Channel {}: {:?}", name, data.iter().collect::<Vec<_>>());
                    }
                }
            }
        }
    }

    let handler = Arc::new(StdoutHandler);

    device.new_data.subscribe(handler.clone());

    device.start(config)?;

    // Wait until carriage return is pressed in console
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}
