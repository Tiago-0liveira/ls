use std::env;
use std::fs;
use std::ffi;
use ansi_term::Colour;
use ansi_term::Style;

#[allow(dead_code)]/* ignore non-used consts */
mod special_chars;

fn main() {
    let path = env::current_dir().unwrap();
    
    let read_dir = match fs::read_dir(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e.kind());
            std::process::exit(1);
        },
    };
    println!("{} Reading {}", special_chars::CORNER_DOWN_RIGHT, Colour::White.bold().paint(path.to_str().unwrap()));

    let mut paths = read_dir
        .map(|x| {
            let s = x.unwrap();
            (s.file_name(), fs::metadata(s.path()).unwrap().is_dir())
        })
        .collect::<Vec<(ffi::OsString, bool)>>();
    paths.sort_by(|a,b| a.1.cmp(&b.1).reverse());
    for (file_name,is_dir) in paths {
        if is_dir {
            let dir_entries_count: i32 = match fs::read_dir(&path.join(&file_name)) {
                Ok(file) => file.count() as i32,
                Err(_) => -1,
            };
            let file_name_to_print = Style::new().bold().paint(if dir_entries_count == 0 {
                Colour::RGB(190,190,190).paint(file_name.to_str().unwrap()).to_string()
            } else {file_name.to_str().unwrap().to_string()});
            let dir_entries_string;
            if dir_entries_count == -1 {
                dir_entries_string = Colour::RGB(200,0,0).bold().paint("?");
            } else if dir_entries_count == 0 {
                dir_entries_string = Colour::RGB(190,190,190).bold().paint(dir_entries_count.to_string());
            } else {
                dir_entries_string = Colour::Green.bold().paint(dir_entries_count.to_string());
            };
            println!("{} {} ({})", special_chars::VERTICAL_BAR_RIGHT, file_name_to_print, dir_entries_string);
        } else {
            println!("{} {}", special_chars::VERTICAL_BAR, file_name.to_str().unwrap());
        }
    }
}
