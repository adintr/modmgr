use ini::Ini;
use std::env;
use std::io;
use std::io::Write;

fn input_num() -> i32 {
    loop
    {
        let mut line = String::new();        
        io::stdin().read_line(&mut line).unwrap();
        let num: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("only interger input allowed: ");
                io::stdout().flush().unwrap();
                continue;
            }
        };
        return num;        
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

    print!("Please select a game:");
    io::stdout().flush().unwrap();
    let sel = input_num();
    println!("select {}", sel);
}
