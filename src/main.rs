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

    let mut i = 0;
    for contact in reader {
        let c = contact?;
        let version = c.properties.into_iter().find(|p| p.name == "VERSION");

        let value = match version {
            Some(v) => match v.value {
                Some(val) => val,
                None => String::new(),
            },
            None => String::new(),
        };

        println!("{}", value);

        i = i + 1;
        if i > 0 {
            break;
        };
    }

    Ok(())
}
