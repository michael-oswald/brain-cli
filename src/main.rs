use std::io;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::fs;
use termimad::{ MadSkin, rgb};
use std::io::Write;
use std::process::Command;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let brain_path = "./.brain-cli/brain.md";
    if args.len() < 2 { // no extra params passed, just save the memory
        save_memory(brain_path);
    } else { // at least one param is passed lets check what it is and do stuff
        let first_arg = &args[1];
        if first_arg.eq("list") {
            // list the brain here
            let brain_file_exists = Path::new(brain_path).exists();
            if !brain_file_exists {
                println!("🧠 (Brain): invalid command, please try again");
                std::process::exit(1);
            }

            let file_contents = fs::read_to_string(brain_path)
                .expect("error occurred, could not read brain file");

            println!("🧠💡 Brain Contents Below 🧠💡");
            let skin = make_terminal_skin();
            skin.print_text(file_contents.as_str());

        } else if first_arg.eq("open") {
            Command::new("open")
                .arg("-e")
                .arg(brain_path)
                .spawn()
                .expect("open command failed to run");
        } else if first_arg.eq("find") {
            //Validation here to make sure user entered a second param
            if args.len() < 3 {
                println!("🧠: Please pass a str arg to the find command like so: `brain find <Your Search String>`");
                std::process::exit(1);
            }

            //grab the string user wants to search by:
            let search_str = &args[2];

            let file_contents = fs::read_to_string(brain_path)
                .expect("error occurred, could not read brain file");

            //grab the string user is trying to find:
            let mut did_find = false;
            for (i, line) in file_contents.lines().enumerate() {
                if line.contains(search_str) {
                    did_find = true;
                    println!("🧠: line: {} => {}", i + 1, line);
                }
            }

            //print out if we didn't find any results in the file.
            if !did_find {
                println!("🧠: ... No results ...");
            }

        } else {
            println!("🧠 (Brain): invalid command, please try again");
            std::process::abort();
        }
    }
}

fn save_memory(file_location: &str) {
    let brain_file_exists = Path::new(file_location).exists();

    if !brain_file_exists { //need to create the file, the write to it
        let path = std::path::Path::new(file_location);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        File::create(file_location).expect("Failed to create file, please try again");
    }

    println!("🧠 (Brain): What do you want to remember?");
    let mut thing_to_remember = String::new();

    io::stdin()
        .read_line(&mut thing_to_remember)
        .expect("Failed to read your request. Please try again"); //this will crash the program if error happens

    // get next input on why to store it, and then save it
    println!("🧠 (Brain): Enter Title of this memory:");

    let mut why_store_this_thing = String::new();
    io::stdin()
        .read_line(&mut why_store_this_thing)
        .expect("Failed to read your request. Please try again");

    let mut md_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_location)
        .unwrap();

    if !brain_file_exists {
        writeln!(md_file,"# Your 🧠 Brain File").expect("Failed to write to file, please try again");
    }

    //now save two lines in the file.
    writeln!(md_file,"## {}", why_store_this_thing).expect("Failed to write to file, please try again");
    writeln!(md_file,"```\n{}```\n", thing_to_remember).expect("Failed to write to file, please try again");

    //randomly pick a different hint msg (between two) to show user
    let range = rand::thread_rng().gen_range(1..4);

    match range {
        // Match a single value
        1 => println!("🧠 (Brain): I've saved the new item.\n Hint: use `brain list` to open all my memories in the terminal 🖥"),
        2 => println!("🧠 (Brain): I've saved the new item.\n Hint: use `brain open` to open up file in your default text editor 🗒"),
        _ => println!("🧠 (Brain): I've saved the new item.\n Hint: use `brain find <text-to-find>` to search for lines in your brain file 🔎"),
    }

}

fn make_terminal_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(246, 189, 0));
    skin
}

