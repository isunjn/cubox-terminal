use std::fs;
use std::io::ErrorKind::NotFound;
use chrono::{naive::NaiveDate, Local};
use getopts::Matches;
use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserCfg {
    api_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    last_date: Option<NaiveDate>,
    count_today: i32,
    count_total: i32,
}

fn load<T: Serialize + DeserializeOwned>(file: &str) -> Result<T, ()> {
    let path = match dirs::home_dir() {
        Some(mut p) => {
            p.push(".cubox");
            p.push(file);
            p.set_extension("json");
            p
        }
        None => return Err(()),
    };
    match fs::read_to_string(path) {
        Ok(content) => Ok(serde_json::from_str::<T>(&content).unwrap()),
        Err(e) if e.kind() == NotFound => match file {
            "config" => {
                let cfg = serde_json::from_str::<T>(r#"{"api_key":""}"#).unwrap();
                store(file, &cfg).unwrap();
                Ok(cfg)
            }
            "data" => {
                let data =
                    serde_json::from_str::<T>(r#"{"count_today":0,"count_total":0}"#).unwrap();
                store(file, &data).unwrap();
                Ok(data)
            }
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

fn store<T: Serialize>(file: &str, content: &T) -> Result<(), ()> {
    let mut path = match dirs::home_dir() {
        Some(mut p) => {
            p.push(".cubox");
            p
        }
        None => return Err(()),
    };
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    path.push(file);
    path.set_extension("json");
    let content = serde_json::to_string_pretty(content).unwrap();
    fs::write(path, content).unwrap();
    Ok(())
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
        .optflagopt("k", "apikey", "Set up your cubox API key", "KEY")
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
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(true);
    }

    if matches.opt_present("h") {
        print_usage(opts);
        return Ok(true);
    }

    if matches.opt_present("c") {
        let mut data: UserData = load("data").unwrap(); // TODO maybe oop style is better
        let today = Local::now().naive_local().date();
        if let Some(last_date) = data.last_date {
            if today > last_date {
                data.count_today = 0;
            }
        }
        println!(
            "Saved via API using cu:\nToday: {}\nTotal: {}",
            data.count_today, data.count_total
        );
        return Ok(true);
    }

    if let Some(api_key) = matches.opt_str("k") {
        let cfg = UserCfg {
            api_key: Some(api_key),
        };
        if let Err(_) = store("config", &cfg) {
            return Err("Fail to store API key");
        };
        println!("✓ API key set.");
        return Ok(true);
    } else {
        if matches.opt_present("k") {
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
    let cfg: UserCfg = match load("config") {
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

pub fn check_response(resp: CuboxResponse) -> Result<(), String> {
    if let 200 = resp.code {
        let mut data: UserData = load("data").unwrap();
        let today = Local::now().naive_local().date();
        if let Some(last_date) = data.last_date {
            if today > last_date {
                data.count_today = 0;
            }
        }
        data.last_date = Some(today);
        data.count_today += 1;
        data.count_total += 1;
        store("data", &data).unwrap();
        println!("✓ Saved!");
        Ok(())
    } else {
        Err(format!("✕ Save failed: {}", resp.message))
    }
}
