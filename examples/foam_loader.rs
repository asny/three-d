use dust::loader;
use std::io::BufRead;
use std::str;

pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(String)
{
    let on_l = |text: Box<BufRead>|
    {
        let mut meta_data = MetaData {format: FORMAT::NONE, file_type: FILETYPE::NONE};
        let mut data= Vec::new();
        let mut should_read_data = false;
        for line in text.lines()
        {
            let l = line.unwrap();
            let mut words: Vec<&str> = l.trim().split(' ').map(|s| s.trim()).collect();
            words.retain(|&i| i != "");

            if words.len() > 0
            {
                if *words.first().unwrap() == "//"
                {
                    should_read_data = !should_read_data;
                }
                if should_read_data
                {
                    match meta_data.file_type {
                        FILETYPE::OWNER => {read_data::<u32>(&words, &mut data);},
                        FILETYPE::NONE => {}
                    }
                }
                else {
                    read_meta_data(&words, &mut meta_data);
                }
            }
        }
        println!("Format: {:?}", meta_data.format);
        println!("Data: {:?}", data);
        on_load(String::from(""));
    };
    loader::load(name, on_l);
}

#[derive(Debug)]
enum FORMAT {ASCII, BINARY, NONE}

#[derive(Debug)]
enum FILETYPE {OWNER, NONE}

struct MetaData {
    format: FORMAT,
    file_type: FILETYPE
}

fn read_meta_data(words: &Vec<&str>, meta_data: &mut MetaData)
{
    if words.len() > 1
    {
        match *words.first().unwrap() {
            "format" => { meta_data.format = match words[1] {
                "ascii;" => FORMAT::ASCII,
                "binary;" => FORMAT::BINARY,
                _ => FORMAT::NONE
            }},
            "object" => { meta_data.file_type = match words[1] {
                "owner;" => FILETYPE::OWNER,
                _ => FILETYPE::NONE
            }},
            &_ => {}
        }
    }
}

fn read_data<T>(words: &Vec<&str>, data: &mut Vec<T>) where T: str::FromStr
{
    for s in words {
        match s.parse::<T>() {
            Ok(i) => data.push(i),
            Err(..) => {},
        };
    }
}
