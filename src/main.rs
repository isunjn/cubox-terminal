use cubox::*;
use std::process;

fn main() {
    let (opts, matches) = get_matches().unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);
        process::exit(1);
    });

    let is_done = handle_options(opts, &matches).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);
        process::exit(1);
    });
    if is_done { return }

    let cubox_request = build_request(matches).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);
        process::exit(1);
    });

    let cubox_response = send_request(cubox_request).unwrap_or_else(|err| {
        eprintln!("[Error] {}", err);
        process::exit(1);
    });

    match cubox_response.code {
        200 => {
            println!("✓ Saved!");
        }
        _ => {
            eprintln!("✕ Save failed: {}", cubox_response.message);
            process::exit(1);
        }
    }
}
