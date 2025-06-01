use std::io::{self, Write};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread::sleep;

macro_rules! print_char_with_delay {
    ($handle:expr, $c:expr) => {
        {
            let s = $c.to_string();
            $handle.write_all(s.as_bytes()).unwrap();
            $handle.flush().unwrap();
            let now = SystemTime::now();
            let timestamp_ms_char = now
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string()
                .chars()
                .collect::<Vec<char>>();
            let rnd_ms = timestamp_ms_char
                .iter()
                .rev()
                .take(2)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            sleep(Duration::from_millis(rnd_ms));
        }
    }
}

macro_rules! hello {
    () => {
        {
            hello!("World");
        }
    };
    ($name:expr) => {
        {
            let hello_world_message = format!("Hello, {}!", $name);
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            for c in hello_world_message.chars() {
                print_char_with_delay!(handle, c);
            }
            handle.write_all(b"\n").unwrap();
            handle.flush().unwrap();
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        hello!(args[1].as_str());
        return;
    }
    hello!()
}