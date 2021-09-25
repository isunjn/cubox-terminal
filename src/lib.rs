use confy;
use getopts::Matches;
use reqwest;
use serde::{Deserialize, Serialize};

/// User configuration
#[derive(Serialize, Deserialize)]
struct UserCfg {
    api_key: Option<String>,
}

impl std::default::Default for UserCfg {
    fn default() -> Self {
        Self { api_key: None }
    }
}

/// Cubox request
#[derive(Serialize, Debug)]
pub struct CuboxRequest {
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
    tags: Option<Vec<String>>,
}

/// Cubox response
#[derive(Deserialize, Debug)]
pub struct CuboxResponse {
    pub message: String,
    pub code: i32,
}

//------------------- pravite function -----------------------

fn print_usage(opts: getopts::Options) {
    let brief = "cu: Take cubox memo in terminal";
    let usage =
        "Usage: cu [options]\n       cu some memo text @folder ^title ::tag1 ::tag2 %description";
    let help = format!("{}\n\n{}", brief, usage);
    print!("{}", opts.usage(&help));
}

//------------------- public function ------------------------

pub fn get_matches() -> Result<(getopts::Options, Matches), String> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optflag("v", "version", "Show the version of cu")
        .optflag("h", "help", "Show the help message of cu")
        .optflag("c", "count", "Show how many times API was used")
        .optflagopt("k", "apikey", "Set up your cubox API key", "API-KEY")
        .optflagopt(
            "l",
            "url",
            "Take an url to create a bookmark\n ranther than a memo",
            "URL",
        );

    match opts.parse(&args[1..]) {
        Ok(m) => Ok((opts, m)),
        Err(f) => Err(f.to_string()),
    }
}

pub fn handle_options(opts: getopts::Options, matches: &Matches) -> Result<bool, &'static str> {
    if matches.opt_present("v") {
        println!("v0.0.1");
        return Ok(true);
    }

    if matches.opt_present("h") {
        print_usage(opts);
        return Ok(true);
    }

    if matches.opt_present("c") {
        //TODO: count_cacl()
        let today_count = 0;
        let total_count = 0;
        println!(
            "Saved via API using cu:\nToday: {}\nTotal: {}",
            today_count, total_count
        );
        return Ok(true);
    }

    if let Some(api_key) = matches.opt_str("apikey") {
        let key = api_key.clone();
        confy::store(
            "cubox",
            UserCfg {
                api_key: Some(api_key),
            },
        )
        .unwrap(); // TODO
        println!("✓ API key set to: {}", key);
        return Ok(true);
    } else {
        if matches.opt_present("apikey") {
            return Err("Expect API key: cu --apikey [API-KEY]");
        }
    }

    Ok(false)
}

pub fn build_request(matches: Matches) -> Result<CuboxRequest, &'static str> {
    let mut cubox_request = CuboxRequest {
        req_type: "memo".to_string(),
        content: "".to_string(),
        title: None,
        description: None,
        folder: None,
        tags: None,
    };

    for s in matches.free.iter() {
        if let Some(folder) = s.strip_prefix("@") {
            cubox_request.folder = Some(folder.to_owned());
            continue;
        }
        if let Some(title) = s.strip_prefix("^") {
            cubox_request.title = Some(title.to_owned());
            continue;
        }
        if let Some(tag) = s.strip_prefix("::") {
            match cubox_request.tags {
                Some(ref mut tags) => {
                    tags.push(tag.to_owned());
                }
                None => {
                    cubox_request.tags = Some(vec![tag.to_owned()]);
                }
            }
            continue;
        }
        if let Some(description) = s.strip_prefix("%") {
            cubox_request.description = Some(description.to_owned());
            continue;
        }
        cubox_request.content.push_str(&s);
        cubox_request.content.push(' ');
    }

    if matches.opt_present("url") {
        match matches.opt_str("url") {
            Some(url) => {
                cubox_request.req_type = "url".to_string();
                cubox_request.content = url;
            }
            None => {
                return Err("Expect an URL: 'cu --url [URL]'");
            }
        }
    }

    if cubox_request.content.is_empty() {
        return Err("Ops! Nothing to save.");
    }

    Ok(cubox_request)
}

pub fn send_request(req: CuboxRequest) -> Result<CuboxResponse, &'static str> {
    let cfg: UserCfg = match confy::load("cubox") {
        Ok(cfg) => cfg,
        Err(_) => return Err("Fail to load config file."),
    };

    let api_key = match cfg.api_key {
        Some(api_key) => api_key,
        None => return Err("API key not found! Use 'cu --apikey [API-KEY]' to set."),
    };

    let cubox_api_url = "https://cubox.pro/c/api/save/".to_string() + &api_key;

    let client = reqwest::blocking::Client::new();
    let resp = match client.post(cubox_api_url).json(&req).send() {
        Ok(resp) => resp,
        Err(_) => return Err("Fail to send request"),
    };

    match resp.json::<CuboxResponse>() {
        Ok(resp) => Ok(resp),
        Err(_) => Err("Fail to parse response to json"),
    }
}
