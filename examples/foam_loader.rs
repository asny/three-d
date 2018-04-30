use dust::loader;
use std::io::BufRead;
use std::str;

pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(String)
{
    let on_l = |text: Box<BufRead>|
    {
        let mut meta_data = MetaData {format: FORMAT::NONE, file_type: FILE_TYPE::NONE};
        let mut data= Vec::new();
        for line in text.lines()
        {
            let l = line.unwrap();

            //println!("{}", l);
            let mut words: Vec<&str> = l.trim().split(' ').map(|s| s.trim()).collect();
            words.retain(|&i| i != "");

            if words.len() > 0
            {
                read_meta_data(&words, &mut meta_data);
                read_data::<u32>(&words, &mut data);
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
enum FILE_TYPE {OWNER, NONE}

struct MetaData {
    format: FORMAT,
    file_type: FILE_TYPE
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
                "owner;" => FILE_TYPE::OWNER,
                _ => FILE_TYPE::NONE
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

/*pub fn read_data(text: &str, format: &str, numberOfComponents: u32, dataformat: &str, faces: bool)
{
    /*let numberOfAttributes = parseInt( state.parsedString[ 0 ].split('((')[ 0 ], 10 );
    let attribute = new Array( numberOfAttributes * numberOfComponents );

    if(format == "ascii")
    {
        let mut lines = text.lines();
        while lines {
            
        }
        let allDataOnOneLine = state.parsedString.length > numberOfComponents;

        for i in 0..numberOfAttributes
            {
                let data;
                if (allDataOnOneLine)
                    {
                        let index = numberOfComponents * i;
                        data = state.parsedString.slice(index, index + 3);
                    } else {
                    state = this.findString(buffer, state.next);
                    data = state.parsedString;
                }

                for j in 0..numberOfComponents
                    {
                        let value = data[j].replace('((',
                        '(').replace('))', ')');
                        if (j == = 0 && value.split('(').length > 1)
                            {
                                value = value.split('(')[1];
                            }
                        value = value.replace('(', '').replace(')', '')
                        attribute[numberOfComponents * i + j] = parseFloat(value, 10);
                    }
            }
    }*/
}*/