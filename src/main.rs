use cubox::*;

fn main() {
    if let Err(e) = run() {
        eprintln!("[Error] {}", e);
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
    match cubox_response.code {
        200 => {
            println!("✓ Saved!");
            Ok(())
        }
        _ => Err(format!("✕ Save failed: {}", cubox_response.message)),
    }
}
