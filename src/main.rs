use clap::Parser;
use rand::prelude::*;

mod config;
mod utils;

use config::{AddressType, Config};

#[derive(Debug, Parser)]
struct Args {

    num_traces: u32,
    num_addrs: Option<u32>,
    #[clap(short, long = "config")]
    config_path: Option<String>,
    #[clap(short, action = clap::ArgAction::Count)]
    r_cnt: u8,
    #[clap(short, action = clap::ArgAction::Count)]
    w_cnt: u8,
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI args
    let args = Args::parse();

    // Parse config
    let config_path = match args.config_path {
        Some(path) => path,
        None => "./trace.config".to_string(),
    };
    let config = Config::from_file(&config_path).expect("Error reading config");
    let max_addr = match config.address_type {
        AddressType::Physical => config.pt.physical_pages * config.pt.page_size,
        AddressType::Virtual => config.pt.virtual_pages * config.pt.page_size,
    };

    // Initialize rng
    let mut rng = rand::thread_rng();

    // Create the set of addrs
    let mut addrs = Vec::new();
    let num_addrs = match args.num_addrs {
        Some(n) => n,
        None => args.num_traces,
    };
    for _addr_idx in 0..num_addrs {
        addrs.push(rng.gen_range(0..=max_addr));
    }

    // Create ratio of reads and writes
    let mut wr_ratio = Vec::new();
    if args.r_cnt == 0 {
        wr_ratio.push('r');
    } else {
        for _r in 0..args.r_cnt {
            wr_ratio.push('r');
        }
        for _w in 0..args.w_cnt {
            wr_ratio.push('w');
        }
    }

    // Create trace from addrs
    for _ in 0..args.num_traces {
        let access_type = wr_ratio.choose(&mut rng).unwrap();
        let addr = addrs.choose(&mut rng).unwrap();
        println!("{}:{:08x}", access_type, addr);
    }

    Ok(())
}
