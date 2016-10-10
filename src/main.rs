extern crate rustc_serialize;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use std::env;

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransferParams {
    file_name: String,
    copy_to_path: String
}

fn main() {
    // Get the passed arguments
    let args: Vec<String> = env::args().collect();

    // check to make sure the number of arguments is correct,
    //  if there are not exactly one argument (one plus the program), exit the program.
    if args.len() != 2  {
        // print program usage
        println!("Usage: file_copy_tool <Copy Folder Path>");
        
        // exit the program
        std::process::exit(0);
    }

    let folder_name = args[1].clone();
    
    // call the copy_file function
    copy_file(&folder_name);
}

// copy_file, copies a file to a different location based on the transfer.json file
fn copy_file( folder_path: &str ) {
    
    // create a path to the folder_path and include the transfer.json file
    let temp_path = Path::new(&folder_path);
    let path = temp_path.join("transfer.json");

    // create display object for displaying the file.
    let display = path.display();

    // matching is like error checking in rust.  This match tries to open a file and switches through based on 
    //  what the result is (Err, Ok, etc.)
    let mut file =  match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };
    
    // define a string for the file contents
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) { 
        Err(why) => panic!("Coulding read {}: {}", display, why.description()),
        Ok(_) => { }, // Do nothing
    };

    // create an object that is the decoded json from the transfer.json file
    let decoded: TransferParams = json::decode(&file_contents).unwrap();

    // run the built-in file copy to move the file to the specified destination.
    match fs::copy(decoded.file_name, decoded.copy_to_path) {
        Err(why) => panic!("Couldn't copy the file {}: {}", display, why.description()),
        Ok(_) => println!("File copy complete!")
    };
}