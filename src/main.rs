use std::io;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    if args.len() < 2 {
        save_memory();
    } else {
        let first_arg = &args[1];
        if first_arg.eq("list") {
            //list the brain here
            println!("🧠 (Brain): Fetching all memories.....");
            let brain_file_exists = Path::new("./brain.md").exists();
            if !brain_file_exists {
                println!("🧠 (Brain): invalid command, please try again");
                std::process::abort();
            }

            let file_contents = fs::read_to_string("./brain.md")
                .expect("error occurred, could not read brain file");

            println!("🧠💡🧠💡🧠💡🧠💡🧠💡🧠💡🧠💡🧠💡🧠\n{file_contents}");
        } else {
            println!("🧠 (Brain): invalid command, please try again");
            std::process::abort();
        }
    }
}

fn save_memory() {
    println!("🧠 (Brain): What do you want to remember?");

    let mut thing_to_remember = String::new();

    io::stdin()
        .read_line(&mut thing_to_remember)
        .expect("Failed to read your request. Please try again"); //this will crash the program if error happens

    let brain_file_exists = Path::new("./brain.md").exists();

    if !brain_file_exists { //need to create the file, the write to it
        File::create("./brain.md").expect("Failed to create file, please try again");
    }

    // get next input on why to store it, and then save it
    println!("🧠 (Brain): Enter Title of this memory:");

    let mut why_store_this_thing = String::new();
    io::stdin()
        .read_line(&mut why_store_this_thing)
        .expect("Failed to read your request. Please try again");

    let mut md_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./brain.md")
        .unwrap();

    if !brain_file_exists {
        writeln!(md_file,"# Your 🧠 Brain File").expect("Failed to write to file, please try again");
    }

    //now save two lines in the file.
    writeln!(md_file,"## {}", why_store_this_thing).expect("Failed to write to file, please try again");
    writeln!(md_file,"```\n{}```\n", thing_to_remember).expect("Failed to write to file, please try again");

    println!("🧠 (Brain): I've saved the new item.\n Hint: use `brain list` to open all my memories");
}