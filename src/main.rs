use chrono::prelude::*;
use clap::Parser;
use libosu::data::Mods;
use libosu::replay::Replay;
use std::fs::File;
use std::io::BufWriter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the source replay file
    #[arg(short, long)]
    replay: String,

    /// Path to the output replay file
    #[arg(short, long)]
    output: String,

    /// OPTIONAL: Username to change it to
    #[arg(short, long)]
    name: Option<String>,

    /// OPTIONAL: Mods to change it to
    #[arg(short, long)]
    mods: Option<String>,

    /// OPTIONAL: Date to change it to (see format with '--help')
    #[arg(
        short,
        long,
        long_help = "OPTIONAL: Date to change it to\nFormat: \"%Y-%m-%d %H:%M:%S %#z\"\nExample: \"2023-07-08 16:20:00 +03\" being 4:20pm UTC+3 on July 8th, 2023"
    )]
    date: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut input = File::open(args.replay).unwrap();
    let output = File::create(args.output).unwrap();

    let mut replay = Replay::parse(&mut input).unwrap();

    if let Some(name) = args.name.as_deref() {
        println!("Changing username to: {}", name);
        replay.player_username = name.to_string();
    }

    if let Some(mods) = args.mods.as_deref() {
        println!("Changing mods to: {}", mods);
        replay.mods = Mods::parse_from_str(&mods, "").unwrap();
    }

    if let Some(date) = args.date.as_deref() {
        println!("Changing date and time to: {}", date);
        replay.timestamp = parse_to_tick(date);
    }

    replay.write(BufWriter::new(output)).unwrap();
}

fn parse_to_tick(date_time_str: &str) -> u64 {
    let format = "%Y-%m-%d %H:%M:%S %#z";
    let timestamp = DateTime::parse_from_str(date_time_str, format)
        .unwrap()
        .timestamp();
    (timestamp as u64 * 10000000) + 621355968000000000
}
