use cubox::*;
use colored::Colorize;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.red());
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let (opts, matches) = get_matches()?;
    let done = handle_options(opts, &matches)?;
    if done {
        return Ok(());
    }
    let cubox_request = build_request(matches)?;
    let cubox_response = send_request(cubox_request)?;
    check_response(cubox_response)?;
    Ok(())
}
