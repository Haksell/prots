extern crate reqwest;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ligand = std::env::args().nth(1).unwrap();
    println!("{}", ligand);
    return Ok(());
    let ligand_url = format!("http://ligand-expo.rcsb.org/reports/{}/{}/{}_ideal.pdb", &ligand[0..1], ligand, ligand);
    let res = reqwest::blocking::get(&ligand_url)?;
    println!("{} | {}", ligand, if res.status().is_success() {"Y"} else {"N"});
    Ok(())
}
