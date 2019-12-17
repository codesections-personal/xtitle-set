use clap::{crate_authors, crate_name, crate_version, App};
use run_script::run_script;
fn main() {
    let cli = App::new(crate_name!())
        .about("Sets the xtitle for a terminal via escape characters.")
        .version(crate_version!())
        .author(crate_authors!())
        .arg("<TITLE> 'The new xtitle to set for the current terminal'")
        .get_matches();

    let (_, out, _) = run_script!(format!(
        r#"[ $(uname -s) = Linux ] && exec echo -e "\033]1;{title}\007\033]2;{title}\007\c""#,
        title = cli.value_of("TITLE").expect("required by clap")
    ))
    .unwrap();
    print!("{}", out);
}
