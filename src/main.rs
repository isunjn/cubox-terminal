
use serde::{ Serialize, Deserialize };
use confy;
use getopts;


///////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
struct UserCfg {
    api_key: Option<String>,
}

impl std::default::Default for UserCfg {
    fn default() -> Self {
        Self { api_key: None }
    }
}


///////////////////////////////////////////////

#[derive(Serialize)]
struct CuboxRequest {
    #[serde(rename = "type")]
    req_type: String,

    content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    folder: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>
}


#[derive(Deserialize, Debug)]
struct CuboxResponse {
    message: String,
    code: i32,
}




fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optflag("v", "version", "Show the version of cu")
        .optflag("h", "help", "Show the help message of cu")
        .optflag("c", "count", "Show how many times API was used")
        .optflagopt("k", "apikey", "Set up your cubox API key", "<API-KEY>")
        .optflagopt("l", "url", "Take an url to create a bookmark\n ranther than a memo", "<URL>");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("v") {
        println!("v0.0.1");
        return;
    }

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    if matches.opt_present("c") {
        let today_count = 0;// TODO
        let total_count = 0;// TODO
        println!("Saved via API using cu:\nToday: {}\nTotal: {}", today_count, total_count);
        return;
    }

    if matches.opt_present("k") {
        match matches.opt_str("k") {
            Some(api_key) => {
                confy::store("cubox", UserCfg { api_key: Some(api_key) }).unwrap(); // TODO
                println!("✓ API key set!");
                return;
            },
            None => { 
                eprintln!("[Error] Expect API key: cu --apikey <API-KEY>");
                return;
            }

        }
    }

    /////////////////////////////////////////////

    let cfg: UserCfg = confy::load("cubox").unwrap();

    let api_key = match cfg.api_key {
        Some(api_key) => api_key,
        None => {
            eprintln!("Error: API key not set!\nPlease use 'cu --apikey <API-KEY>' to set up your API key");
            return;
        }
    };

    println!("Your api key is: {}", api_key);

    //////////////////////////////////////////////////

    let mut memo = "".to_string();
    for s in matches.free {
        memo.push_str(&s);
    }

    let cubox_request = CuboxRequest {
        req_type: "memo".to_string(),
        content: memo,
        title: None,
        description: None,
        folder: None,
        tags: None,
    };

    let cubox_api_url = "https://cubox.pro/c/api/save/".to_string() + &api_key;

    let client = reqwest::blocking::Client::new();
    let resp = client.post(cubox_api_url)
        .json(&cubox_request)
        .send().unwrap();

    let resp = resp.json::<CuboxResponse>().unwrap();

    println!("{:#?}", resp);

}

fn print_usage(opts: getopts::Options) {
    let brief = format!("cu: Take cubox memo in terminal\n\nUsage: cu [options]");
    print!("{}", opts.usage(&brief));
}
