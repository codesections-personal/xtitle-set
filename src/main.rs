use clap::{crate_name, crate_version, App, Arg};

fn main() {
    #[rustfmt::skip]
    let cli = App::new(crate_name!())
        .about("Set the xtitle for the current terminal via escape characters.")
        .version(crate_version!())
        .arg(Arg::with_name("<TITLE>")
             .takes_value(true)
             .help("The new xtitle to set for the current terminal")             
             .required_unless("src"))
        .arg(Arg::from("--src 'Prints this program's source to stdout'"))
        .get_matches();

    if cli.is_present("src") {
        print!("/// main.rs\n{}", include_str!("main.rs"));
    } else {
        print!(
            "\u{1b}]1;{title}\u{7}\u{1b}]2;{title}\u{7}",
            title = cli.value_of("TITLE").expect("required by clap")
        );
    }
}
