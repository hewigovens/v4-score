mod score;
mod v4address;

use std::{
    collections::BinaryHeap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use score::rate_address;
use v4address::V4AddressResult;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a efficient_addresses file path from create2crunch");
        std::process::exit(1);
    }
    process_file(&args[1])?;
    Ok(())
}

fn process_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Use a max heap to track top addresses
    let mut top_addresses = BinaryHeap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
        if parts.len() != 3 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        let result = V4AddressResult {
            salt: parts[0].to_string(),
            score: rate_address(parts[1]),
            address: parts[1].to_string(),
        };

        if result.score == 0 {
            // skip invalid addresses
            continue;
        }

        // Add to top addresses, keeping only top 10
        if top_addresses.len() < 10 {
            top_addresses.push(result);
        } else if let Some(mut top) = top_addresses.peek_mut() {
            if result.score > top.score {
                *top = result;
            }
        }
    }

    println!("Top 10 Addresses:");
    let mut sorted_top = top_addresses.into_sorted_vec();
    sorted_top.reverse(); // Highest scores first
    for (rank, result) in sorted_top.into_iter().enumerate() {
        println!("{rank}. {result}");
    }
    Ok(())
}
