use std::env::args;
use std::fs;
use std::env;
use std::io;
use std::{thread, time};


struct Files {
    from: String,
    to: String,
}

fn main() -> io::Result<()> {
    //let args: Vec<String> = env::args().collect();
    let files = Files::new;

    println!("Welcome to filer..");
    println!("Please enter a filename: ");

    //println!("Copying {} to {}", files.from, files.to);




    let mut from = String::new();
    let mut to = String::new();
    io::stdin().read_line(&mut from)?;
    io::stdin().read_line(&mut to)?;
    println!{"Copying {} to {}", from, to};
    fs::copy(new.to, files.to)?;

    Ok(())


}

impl Files {
    fn new(args: &[String]) -> Files {
    let from = args[1].clone();
    let to = args[2].clone();

    Files {from, to}
    }
}