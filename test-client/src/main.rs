use std::collections::BTreeMap;

use anyhow::bail;
use clap::Parser;
use klipper::Connection;
use moonraker::{prelude::*, types::server::GcodeStore};
use tokio::io::{AsyncBufReadExt, BufReader, stdin};
use tracing_subscriber::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, short, env = "MOONRAKER_SERVER")]
    server: String,
}

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv_override()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async_main(args))
}

async fn async_main(args: Args) -> anyhow::Result<()> {
    let mut klipper = Connection::connect(args.server).await?;

    if !klipper.client.is_connected() {
        bail!("Websocket did not connect");
    }

    let task = klipper.start();
    let client = klipper.client.clone();

    let server_info = client.get_server_info().await?;
    println!(
        "Connected to Moonraker version {}",
        server_info.moonraker_version
    );

    let idenfication = client
        .identify_connection(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            ClientType::Other,
            env!("CARGO_PKG_HOMEPAGE"),
        )
        .await?;
    println!("{:?}", idenfication);

    let server_config = client.get_server_config().await?;
    println!(
        "  Configuration files: {}",
        server_config
            .files
            .iter()
            .map(|file| file.filename.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let GcodeStore { gcode_store } = client.get_gcode_store(Some(5)).await?;
    if !gcode_store.is_empty() {
        println!(
            "  Last Gcode: {}",
            gcode_store.first().map(|g| g.message.clone()).unwrap()
        );
    }

    let temperature_store = client.get_temperature_store(false).await?;
    println!("  Temperatures:");
    for (sensor_name, sensor) in temperature_store.sensors {
        println!(
            "    {}: {:?}",
            sensor_name,
            sensor.temperatures.iter().take(10).collect::<Vec<_>>()
        );
    }

    let klippy_info = client.get_printer_info().await?;
    println!("{klippy_info:?}");

    klipper
        .subscribe_objects(BTreeMap::from([
            ("toolhead", None),
            ("motion_report", None),
            ("extruder", None),
            ("pause_resume", None),
            ("idle_timeout", None),
        ]))
        .await?;

    let objects = client.list_printer_objects().await?;
    println!("Objects: {:?}", objects.objects);
    klipper
        .subscribe_objects(
            objects
                .objects
                .iter()
                .filter(|obj| {
                    obj.starts_with("temperature_sensor ")
                        || obj.starts_with("heater")
                        || obj.starts_with("extruder")
                })
                .map(|obj| (obj, None))
                .collect(),
        )
        .await?;

    println!("Press enter to stop");
    wait_for_input().await;

    Ok(task.stop().await?)
}

async fn wait_for_input() {
    let stdin = stdin();
    let stdin = BufReader::new(stdin);
    let _ = stdin.lines().next_line().await;
}
