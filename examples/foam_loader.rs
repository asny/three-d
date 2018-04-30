use dust::loader;
use std::io::{BufRead, Lines};
use std::str;

pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(String)
{
    let on_l = |text: Box<BufRead>|
    {
        println!("");
        println!("Loading: {}", name);
        let mut meta_data = MetaData {format: FORMAT::NONE, file_type: FILETYPE::NONE};
        let mut lines_iter = text.lines();
        while true
        {
            let line = lines_iter.next().unwrap().unwrap();
            let mut words: Vec<&str> = line.trim().split(' ').map(|s| s.trim()).collect();
            words.retain(|&i| i != "");

            if words.len() > 0
            {
                if *words.first().unwrap() == "//"
                {
                    break;
                }
                read_meta_data_into(&words, &mut meta_data);
            }
        }
        match meta_data.file_type {
            FILETYPE::OWNER => { let data = read_data::<u32>(&mut lines_iter); on_load(String::from(format!("Data: {:?}", data))); },
            FILETYPE::POINTS => {let data = read_data::<f32>(&mut lines_iter); on_load(String::from(format!("Data: {:?}", data))); },
            FILETYPE::NONE => {}
        };
        println!("Format: {:?}", meta_data.format);
    };
    loader::load(name, on_l);
}

/*fn get_words(line: String) -> Vec<&str>
{
    let mut words: Vec<&str> = line.trim().split(' ').map(|s| s.trim()).collect();
    words.retain(|&i| i != "");
    words
}*/

fn read_data<T>(lines_iter: &mut Lines<Box<BufRead>>) -> Vec<T> where T: str::FromStr
{
    let mut data = Vec::new();
    loop
    {
        let line = lines_iter.next().unwrap().unwrap();
        let mut words: Vec<&str> = line.trim().split(' ').map(|s| s.trim()).collect();
        words.retain(|&i| i != "");

        if words.len() > 0
        {
            if *words.first().unwrap() == "//"
            {
                break;
            }
            read_data_into(&words, &mut data);
        }
    }
    data
}

#[derive(Debug)]
enum FORMAT {ASCII, BINARY, NONE}

#[derive(Debug)]
enum FILETYPE {POINTS, OWNER, NONE}

struct MetaData {
    format: FORMAT,
    file_type: FILETYPE
}

fn read_meta_data_into(words: &Vec<&str>, meta_data: &mut MetaData)
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
                "points;" => FILETYPE::POINTS,
                _ => FILETYPE::NONE
            }},
            &_ => {}
        }
    }
}

fn read_data_into<T>(words: &Vec<&str>, data: &mut Vec<T>) where T: str::FromStr
{
    for s in words {
        match s.parse::<T>() {
            Ok(i) => data.push(i),
            Err(..) => {},
        };
    }
}
