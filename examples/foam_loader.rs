use dust::loader;
use std::io::BufRead;
use std::str;

pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(String)
{
    let on_l = |text: Box<BufRead>|
    {
        let mut meta_data = MetaData {format: FORMAT::ASCII};
        for line in text.lines()
        {
            let l = line.unwrap();

            //println!("{}", l);
            let mut words: Vec<&str> = l.trim().split(' ').map(|s| s.trim()).collect();
            words.retain(|&i| i != "");

            read_meta_data(&words, &mut meta_data);

            for s in words {
                println!("Word: {}", s);
                let trimmed = s.trim();
                match trimmed.parse::<u32>() {
                    Ok(i) => println!("your integer input: {}", i),
                    Err(..) => {},
                };
            }

        }
        println!("Format: {:?}", meta_data.format);
        on_load(String::from(""));
    };
    loader::load(name, on_l);
}

#[derive(Debug)]
enum FORMAT {ASCII, BINARY}

struct MetaData {
    format: FORMAT
}

fn read_meta_data(words: &Vec<&str>, meta_data: &mut MetaData)
{
    if words.len() > 1
    {
        match *words.first().unwrap() {
            "format" => { println!("Format: {}", words[1]); meta_data.format = if words[1] == "ascii;" { FORMAT::ASCII } else { FORMAT::BINARY }  },
            &_ => {}
        }
    }
}

pub fn read_data(text: &str, format: &str, numberOfComponents: u32, dataformat: &str, faces: bool)
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
}