#[macro_use]
extern crate clap;
extern crate wsy;

use std::io::stdin;
use wsy::util::options::Options;
use wsy::network::ws::connect;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let mut opts = Options::default();

    if let Some(url) = matches.value_of("URL") {
        opts.url = String::from(url);
    }

    opts.verbosity = matches.occurrences_of("verbose") as u8;

    let sender = connect(opts).unwrap();

    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                sender.send(input);
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}
