use dust::loader;


pub fn load<F>(name: &str, mut on_load: F) where F: FnMut(String)
{
    let on_l = |data: Vec<u8>|
    {
        let str = String::from_utf8(data).unwrap();
        let trimmed = str.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => println!("your integer input: {}", i),
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        on_load(trimmed.to_string());
    };
    loader::load(name, on_l);
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