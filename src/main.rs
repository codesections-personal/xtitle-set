use clap::{crate_authors, crate_name, crate_version, App};
use run_script::run_script;
fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    let args = std::env::args().skip(1).collect::<String>();
    let (_, out, _) = run_script!(format!(
        r#"[ $(uname -s) = Linux ] && exec echo -e "\033]1;{args}\007\033]2;{args}\007\c""#,
        args = args
    ))
    .unwrap();
    println!("{}", out);
}
