extern crate rustc_serialize;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs;
use std::fs::metadata;
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
        println!("Usage: file_copy_tool <Root Folder Path>");
        
        // exit the program
        //std::process::exit(0);
    }

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        //println!("Name: {}", path.unwrap().path().display())
        let filename = path.unwrap().path().as_os_str();

        println!("{}", filename.to_str());
        //let attr = fs::metadata(path.unwrap().path().display()).unwrap();

        //if attr.is_dir() {
            //println!("{}, is a directory!", path.unwrap().path().display());
        //}
        println!("Name: {}", path.unwrap().path().display());
    }

    let startFolder = ".";
    copyFile(startFolder);

    /*
    let temppath = Path::new(&args[1]); //"testfile.json");
    let path = temppath.join("testfile.json");

    let display = path.display();

    //let path = Path::new("testfile.json");
    //let display = path.display();

    // matching is like error checking in rust.  This match tries to open a file and switches through based on 
    //  what the result is (Err, Ok, etc.)
    let mut file =  match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };//try!(File::open("testfile.json"));

    // define a string for the file contents
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) { 
        Err(why) => panic!("Coulding read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains: \n{}", display, file_contents),
    };

    //try!(file.read_to_string(&mut fileContents));

    //println!("contents: {}", fileContents);

    let decoded: TransferParams = json::decode(&file_contents).unwrap();

    //println!("file_name: {}", decoded.file_name);
    //println!("Copy path: {}", decoded.copy_to_path);

    //println!("Trying to copy {} to {}", decoded.file_name, decoded.copy_to_path);

    match fs::copy(decoded.file_name, decoded.copy_to_path) {
        Err(why) => panic!("Couldn't copy the file {}: {}", display, why.description()),
        Ok(_) => println!("File copy complete!")
    };
    */
}

fn copyFile( folder_path: &str ) {
    
    let temppath = Path::new(&folder_path); //"testfile.json");
    let path = temppath.join("testfile.json");

    let display = path.display();

    // matching is like error checking in rust.  This match tries to open a file and switches through based on 
    //  what the result is (Err, Ok, etc.)
    let mut file =  match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };//try!(File::open("testfile.json"));

    // define a string for the file contents
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) { 
        Err(why) => panic!("Coulding read {}: {}", display, why.description()),
        Ok(_) => { }, // Do nothing // print!("{} contains: \n{}", display, file_contents),
    };

    let decoded: TransferParams = json::decode(&file_contents).unwrap();

    match fs::copy(decoded.file_name, decoded.copy_to_path) {
        Err(why) => panic!("Couldn't copy the file {}: {}", display, why.description()),
        Ok(_) => println!("File copy complete!")
    };
}
