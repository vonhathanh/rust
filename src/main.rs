use clap::{command, Arg};

fn main() {
    let matches = command!()
        .arg(Arg::new("name").short('n').long("name")).get_matches();
    println!("name: {:?}", matches.get_one::<String>("name"));
}
