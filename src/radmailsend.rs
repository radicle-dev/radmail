use mailparse::*;
use std::io;

struct RadMail {
    to: String,
    raw: String,
}

fn get_mail() -> RadMail {
    let mut raw = String::new();
    io::stdin().read_to_string(&mut raw);
    match parse_mail(raw) {
        Ok(parsed) =>
            match parsed.headers.get_first_value("To") {
                Ok(Some(to)) => RadMail {to, raw },
                Err(err) => panic!("Missing 'To' header"),
            },
        Err(err) => panic!("Error parsing email: {:?}", err),
    }
}

fn main() {
    println!("hi");
}
