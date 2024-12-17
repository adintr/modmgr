use ini::Ini;
use std::env;
use std::io;
use std::io::Write;
use colored::Colorize;

fn input_num(min: i32, max:i32, title: &str) -> i32 {
    loop {
        print!("{}({}~{}): ", title, min, max);
        io::stdout().flush().unwrap();        
        let mut line = String::new();        
        io::stdin().read_line(&mut line).unwrap();
        match line.trim().parse() {
            Ok(num) => {
                if num >= min && num <= max {
                    break num;
                } else {
                    println!("input must be {} ~ {}", min, max.to_string().blue());
                }
            }
            Err(_) => println!("{}", "only interger input allowed.".red())
        };    
    }
}

fn main() {
    let exe_file = env::current_exe().unwrap();
    let exe_dir = exe_file.parent().unwrap();
    let ini_file = exe_dir.join("modmgr.ini");
    
    let conf = Ini::load_from_file(ini_file).unwrap();
    let i = 1;

    for (sec, _prop) in &conf {
        if sec.is_none() {
            continue;
        }

        println!("{}: {}", i, sec.unwrap());
    }

    let sel = input_num(1, 2, "Please select a game");
    println!("select {}", sel);
}
