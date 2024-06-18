extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ligand = std::env::args().nth(1).expect("Please provide a ligand name as an argument");
    println!("{}", ligand);

    let ligand_url = format!("http://ligand-expo.rcsb.org/reports/{}/{}/{}_ideal.pdb", &ligand[0..1], ligand, ligand);
    let res = reqwest::blocking::get(&ligand_url)?;

    if res.status().is_success() {
        let text = res.text()?;
        let lines: Vec<String> = text.trim().lines().map(|line| line.to_string()).collect();
        println!("{:?}", lines);
    }

    Ok(())
}
