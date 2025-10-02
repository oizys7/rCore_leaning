use std::env;
use std::path::Path;

fn main() {
    let path = if env::args().count() < 2 {
        ".".to_string()
    } else {
        // 使用第一个参数
        env::args().nth(1).unwrap()
    };

    if !Path::new(&path).exists() {
        eprintln!("Error: '{}' does not exist", path);
        std::process::exit(1);
    }

    let entries = std::fs::read_dir(&path).expect("无法读取目录");
    for entry in entries {
        println!("{}", entry.unwrap().file_name().to_string_lossy());
    }
}