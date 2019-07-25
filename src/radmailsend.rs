use mailparse::*;
use std::io::{self, Read};

#[derive(Debug)]
struct RadMail {
    to: String,
    raw: [u8],
}

fn get_mail() -> &'static RadMail {
    let mut raw = &[];
    io::stdin().read(raw);
    match parse_mail(raw) {
        Ok(parsed) =>
            match parsed.headers.get_first_value("To") {
                Ok(Some(to)) => RadMail {to: to, raw: raw },
                Ok(None) => panic!("No one in 'To' header"),
                Err(_err) => panic!("Missing 'To' header"),
            },
        Err(err) => panic!("Error parsing email: {:?}", err),
    }
}

fn main() {
    println!("hi");
}
