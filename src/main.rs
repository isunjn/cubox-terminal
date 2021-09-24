use std::process;
// use cubox::*;

fn main() {
    
    let (opts, matches) = match get_matches() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    };

    match handle_options(opts, &matches) {
        Ok(done) => {
            if done { return };
        }
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    }

    let cubox_request = match build_request(&matches) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    };

    let cubox_response = match send_request(cubox_request) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[Error] {}", e);
            process::exit(1);
        }
    };

    match cubox_response.code {
        200 => {
            println!("✓ Saved!");
        }
        _ => {
            eprintln!("✕ Save failed: {}", cubox_response.message);
        }
    }
}

