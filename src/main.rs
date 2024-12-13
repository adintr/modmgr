use ini::Ini;
use std::env;
use std::io;
use std::io::Write;

/*
use std::any::type_name;

fn print_type_of<T>(_: T) {
    println!("type is {}", std::any::type_name::<T>())
}
*/

fn inputNum() -> i32 {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_err() {
        eprintln!("读取输入失败");
        return 0;
    }

    let num: i32 = match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("请输入一个有效的整数");
            return 0;
        }
    };

    return num;
}

fn main() {
    let exe_file = env::current_exe().unwrap();
    let exe_dir = exe_file.parent().unwrap();
    let ini_file = exe_dir.join("modmgr.ini");
    
    let conf = Ini::load_from_file(ini_file).unwrap();
    let i = 1;

    for (sec, prop) in &conf {
        if sec.is_none() {
            continue;
        }

        println!("{}: {}", i, sec.unwrap());
    }

    print!("Please select a game:");
    io::stdout().flush().unwrap();
    let sel = inputNum();
    println!("select {}", sel);
}
