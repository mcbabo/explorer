use std::{env};
use std::ffi::OsString;
use std::fs;
use std::fs::Metadata;

enum FileType {
    File,
    Directory,
    Link,
}

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
                    new_size = new_size/1024.0;
                    count += 1;
                }

                new_size = (new_size * 100.0).round() / 100.0;

                return match count {
                    0 => {
                        new_size.to_string() + &String::from(" bytes")
                    },
                    1 => {
                        new_size.to_string() + &String::from(" kb")
                    },
                    2 => {
                        new_size.to_string() + &String::from(" mb")
                    },
                    3 => {
                        new_size.to_string() + &String::from(" gb")
                    },
                    4 => {
                        new_size.to_string() + &String::from(" tb")
                    },
                    _ => {
                        new_size.to_string() + &String::from(" bytes")
                    }
                }
                /*
                if self.size <= 1024 {
                    self.size.to_string() + &String::from(" bytes")
                } else {
                    (self.size / 1024).to_string() + &String::from(" kb")
                }
                */
            }
            _ => String::from("..."),
        };
    }

    fn show_name(self: &File) -> String {
        return match self.file_type {
            FileType::Directory => String::from("\x1b[34m") + &self.name.to_str().unwrap().to_string() + &" ".repeat(20) + &String::from("\x1b[0m"),
            FileType::File => {
                if self.name.to_str().unwrap().to_string().split(".").last().unwrap() == "exe"{
                    String::from("\x1b[33;32m") + &self.name.to_str().unwrap().to_string() + &String::from("\x1b[0m")
                } else {
                    self.name.to_str().unwrap().to_string()
                }
            }
            _ => self.name.to_str().unwrap().to_string()
        }
    }

    fn show_filetype(self: &File) -> String {
        return match self.file_type {
            FileType::File => String::from("File"),
            FileType::Directory => String::from("Directory"),
            FileType::Link => String::from("Link"),
        };
    }
}

fn pretty_print(files: Vec<File>, max_name_len: usize) {
    //println!("{0: <max_name_len$} | {1: <10} | {2: <10}", "name", "type", "size", max_name_len=max_name_len);
    //let line = "-".repeat(max_name_len+1);
    //println!("{:}+------------+-----------", line);
    for file in files {
        println!("{:}", file.show_name());
        //println!("{0: <max_name_len$} | {1: <10} | {2:}", file.show_name(), file.show_filetype(), &file.show_size(), max_name_len=max_name_len);
    }
}

fn main() -> std::io::Result<()> {
    let raw_files;

    if let Some(arg1) = env::args().nth(1) {
        raw_files = fs::read_dir(&arg1).unwrap();
    } else {
        raw_files = fs::read_dir(env::current_dir().unwrap()).unwrap();
    }

    let mut files: Vec<File> = vec![];

    let mut max_name_len: usize = 10;

    for file in raw_files {
        let mut temp_type: FileType = FileType::File;

        if file.as_ref().unwrap().metadata().unwrap().is_dir() {
            temp_type = FileType::Directory;
        } else if file.as_ref().unwrap().metadata().unwrap().is_symlink() {
            temp_type = FileType::Link;
        } else if file.as_ref().unwrap().metadata().unwrap().is_file() {
            temp_type = FileType::File;
        }

        if file.as_ref().unwrap().file_name().to_str().unwrap().to_string().len() >= max_name_len {
            max_name_len = file.as_ref().unwrap().file_name().to_str().unwrap().to_string().len();
        }

        files.push(File::new(file.as_ref().unwrap().file_name(), temp_type, file.as_ref().unwrap().metadata().unwrap(), file.as_ref().unwrap().metadata().unwrap().len()))
    }

    pretty_print(files, max_name_len);
    /*
    println!("\x1b[31mThis is red text\x1b[0m");
    println!("\x1b[34mThis is blue text\x1b[0m");
    println!("\x1b[33;1mThis is bold yellow text\x1b[0m");
    println!("\x1b[33;32mThis is underlined text\x1b[0m");
    println!("\x1b[5mThis is blinking text\x1b[0m");
    */
    Ok(())
}
