use chrono::{Local, NaiveTime};
use clap::Parser;
use enigo::{Button, Direction::*, Enigo, Key, Keyboard, Mouse, Settings};
use std::thread;
use std::time::Duration;

fn parse_time_chrono(s: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(s, "%H:%M:%S:%f")
        .map_err(|_| format!("Invalid time format '{}'. Expected HH:MM:SS:MS", s))
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //#[arg(short, long, default_value = Some("Man"))]
    //name: String,
    ///time to start clicking
    #[arg(short, long, value_parser = parse_time_chrono)]
    time: NaiveTime,

    ///delay between clicks (ms)
    #[arg(short, long, default_value_t = 50)]
    delay: u64,

    ///press Ctrl+PageDown after each click
    #[arg(short, long, default_value_t = true)]
    pgdn: bool,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let args = Args::parse();
    let time = args.time;
    let delay = args.delay;
    let pgdn = args.pgdn;
    println!("Launched at {}", Local::now().time());
    println!("Will start at {}", &time);
    loop {
        if Local::now().time() >= time {
            for _ in 0..args.count {
                let _ = enigo.button(Button::Left, Click);
                if pgdn {
                    let _ = enigo.key(Key::Control, Press);
                    let _ = enigo.key(Key::PageDown, Click);
                    let _ = enigo.key(Key::Control, Release);
                }
                thread::sleep(Duration::from_millis(delay));
            }
            break;
        }
    }
    println!("Finished at {}", Local::now().time());
}
