use ical::{parser::ParserError, VcardParser};

use std::{
    fs::File,
    io::{BufReader, Error},
};

fn create_reader() -> Result<VcardParser<BufReader<File>>, Error> {
    let file = File::open("./data/Contacts.vcf")?;
    let buffer = BufReader::new(file);
    Ok(VcardParser::new(buffer))
}

fn main() -> Result<(), ParserError> {
    let reader = match create_reader() {
        Ok(r) => r,
        Err(_) => panic!("Something weird happened."),
    };

    for contact in reader {
        match contact {
            Ok(c) => {
                let bday = c.properties.iter().find(|p| p.name == "BDAY");
                let name = c.properties.iter().find(|p| p.name == "FN");

                match bday {
                    Some(_) => match name {
                        Some(n) => println!("{:?}", n.value),
                        None => (),
                    },
                    None => (),
                }
            }
            Err(_) => (),
        };
    }

    Ok(())
}
