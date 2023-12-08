use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Error};

use clap::Parser;

struct TimingStats {
    median: u128,
    average: u128,
    min: u128,
    max: u128,
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "1000")]
    repeats: u32,

    #[clap(short, long)]
    path: String,

    #[clap(short, long, default_value = "false")]
    nanos: bool,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let read_file = |f: &dyn Fn(&str) -> Result<(), Error>| {
        let stats = time_func(
            &|| {
                let _ = f(&args.path);
            },
            args.repeats,
            args.nanos,
        );

        if args.nanos {
            println!("Median: {} ns", stats.median);
            println!("Average: {} ns", stats.average);
            println!("Min: {} ns", stats.min);
            println!("Max: {} ns", stats.max);
        } else {
            println!("Median: {} μs", stats.median);
            println!("Average: {} μs", stats.average);
            println!("Min: {} μs", stats.min);
            println!("Max: {} μs", stats.max);
        }
    };

    println!("read_file_bufreader");
    read_file(&read_file_bufreader);

    println!();

    println!("read_file_read_to_string");
    read_file(&read_file_read_to_string);

    Ok(())
}

fn use_input(_input: String) {}

fn read_file_bufreader(path: &str) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for data in buffered.lines().flatten() {
        use_input(data)
    }

    Ok(())
}

fn read_file_read_to_string(path: &str) -> Result<(), Error> {
    let contents = read_to_string(path)?;

    for data in contents.lines() {
        use_input(data.to_string())
    }

    Ok(())
}

fn time_func(f: &dyn Fn(), repeats: u32, nanos: bool) -> TimingStats {
    let mut times = Vec::new();
    for _ in 0..repeats {
        let start = std::time::Instant::now();
        f();
        let duration = start.elapsed();
        let time = if nanos {
            duration.as_nanos()
        } else {
            duration.as_micros()
        };
        times.push(time);
    }
    times.sort();
    let min = times[0];
    let max = times[times.len() - 1];
    let median = times[times.len() / 2];
    let sum: u128 = times.iter().sum();
    let average = sum / (repeats as u128);
    TimingStats {
        median,
        average,
        min,
        max,
    }
}
