#![allow(unused)]

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest}; 
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(err) = check_args(args) {
        eprintln!("{}", err);
        exit(1);
    }
    
}

fn check_args(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("Invalid number of arguments\nExample: cargo run <hash>".to_owned());
    } 
    
    Ok(())
}
