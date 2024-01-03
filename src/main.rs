use std::{env};
use std::ffi::OsString;
use std::fs;
use std::fs::{Metadata, ReadDir};
use crossterm::terminal::size;

struct CustomTerminal {
    width: u16,
    height: u16,
}

impl CustomTerminal {
    fn new(width: u16, height: u16) -> CustomTerminal {
        return Self { width, height };
    }
}

#[derive(Debug, Clone)]
enum FileType {
    File,
    Directory,
    Link,
}

#[derive(Debug, Clone)]
struct File {
    name: OsString,
    file_type: FileType,
    metadata: Metadata,
    size: u64,
}

impl File {
    fn new(name: OsString, file_type: FileType, metadata: Metadata, size: u64) -> File {
        return Self { name, file_type, metadata, size };
    }

    fn show_size(self: &File) -> String {
        return match self.file_type {
            FileType::File => {
                let mut count: usize = 0;
                let mut new_size: f32 = self.size as f32;
                while new_size >= 1024.0 {
                    new_size = new_size / 1024.0;
                    count += 1;
                }

                new_size = (new_size * 100.0).round() / 100.0;

                return match count {
                    0 => {
                        new_size.to_string() + &String::from(" bytes")
                    }
                    1 => {
                        new_size.to_string() + &String::from(" kb")
                    }
                    2 => {
                        new_size.to_string() + &String::from(" mb")
                    }
                    3 => {
                        new_size.to_string() + &String::from(" gb")
                    }
                    4 => {
                        new_size.to_string() + &String::from(" tb")
                    }
                    _ => {
                        new_size.to_string() + &String::from(" bytes")
                    }
                };
            }
            _ => String::from("..."),
        };
    }

    fn show_name(self: &File) -> String {
        return match self.file_type {
            FileType::Directory => String::from("\x1b[34m") + &self.name.to_str().unwrap().to_string() + &String::from("\x1b[0m"),
            FileType::File => {
                if self.name.to_str().unwrap().to_string().split(".").last().unwrap() == "exe" {
                    String::from("\x1b[33;32m") + &self.name.to_str().unwrap().to_string() + &String::from("\x1b[0m")
                } else {
                    self.name.to_str().unwrap().to_string()
                }
            }
            _ => self.name.to_str().unwrap().to_string()
        };
    }

    fn show_filetype(self: &File) -> String {
        return match self.file_type {
            FileType::File => String::from("File"),
            FileType::Directory => String::from("Directory"),
            FileType::Link => String::from("Link"),
        };
    }
}

fn get_files(dir: ReadDir) -> Vec<File> {
    let mut files: Vec<File> = vec![];

    for file in dir {
        let mut temp_type: FileType = FileType::File;

        if file.as_ref().unwrap().metadata().unwrap().is_dir() {
            temp_type = FileType::Directory;
        } else if file.as_ref().unwrap().metadata().unwrap().is_symlink() {
            temp_type = FileType::Link;
        } else if file.as_ref().unwrap().metadata().unwrap().is_file() {
            temp_type = FileType::File;
        }

        files.push(File::new(file.as_ref().unwrap().file_name(), temp_type, file.as_ref().unwrap().metadata().unwrap(), file.as_ref().unwrap().metadata().unwrap().len()))
    }

    return files;
}

fn can_be_printed(term_width: &u16, files: &Vec<File>, columns: u16, max_name_len: usize) -> bool {
    let lines = 1 + (u16::try_from(files.len()).unwrap() - 1) / columns;

    for i in 0..lines {
        let mut w: usize = 0;
        w += files.get(i as usize).unwrap().name.to_str().unwrap().len() * 2 + (max_name_len - files.get(i as usize).unwrap().name.to_str().unwrap().len());

        for j in (i + lines..u16::try_from(files.len()).unwrap()).step_by(lines as usize) {
            w += 2 + (max_name_len - files.get(i as usize).unwrap().name.to_str().unwrap().len()) + files.get(j as usize).unwrap().name.to_str().unwrap().len()
        }

        if u16::try_from(w).unwrap() > *term_width {
            return false;
        }
    }
    return true;
}

fn main() -> std::io::Result<()> {
    let mut terminal: CustomTerminal = CustomTerminal { width: 0, height: 0 };
    let size = size();

    match size {
        Ok(size) => {
            println!("{}, {}\n", size.0, size.1);
            terminal = CustomTerminal::new(size.0, size.1);
        }
        Err(_) => {}
    }

    let raw_files;

    if let Some(arg1) = env::args().nth(1) {
        raw_files = fs::read_dir(&arg1).unwrap();
    } else {
        raw_files = fs::read_dir(env::current_dir().unwrap()).unwrap();
    }

    let files: Vec<File> = get_files(raw_files);

    let mut max_name_len: usize = 10;

    for file in &files {
        if file.name.len() >= max_name_len {
            max_name_len = file.name.len();
        }
    }

    let mut low = 1;
    let mut high = terminal.width;

    while high - low > 1 {
        let mid = (low + high) / 2;
        let ans = can_be_printed(&terminal.width, &files, mid, max_name_len);
        if ans {
            low = mid;
        } else {
            high = mid;
        }
    }

    let mut columns: usize;
    if can_be_printed(&terminal.width, &files, high, max_name_len) {
        columns = high as usize;
    } else {
        columns = low as usize;
    }

    for y in 0..files.len() / columns {
        for x in 0..columns {
            let file = &files[y * columns + x];
            let current_len = max_name_len - file.name.to_str().unwrap().len();
            if x != columns -1 {
                print!("{}{}", file.show_name(), " ".repeat(current_len + 2));
            } else {
                print!("{}", file.show_name());
            }
        }
        println!();
    }

    let rem = files.len() % columns;

    if rem != 0 {
        for file in &files[files.len() - rem..] {
            let current_len = max_name_len - file.name.to_str().unwrap().len();
            print!("{}{}", file.show_name(), " ".repeat(current_len + 2));
        }
        println!()
    }

    println!();
    Ok(())
}
