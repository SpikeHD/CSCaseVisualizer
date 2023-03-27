use reqwest::{self, header::COOKIE};
use tauri::regex::Regex;
use std::{fs, time::{SystemTime, UNIX_EPOCH}};
use scraper;
use json;

static BASE_URL: &str = "https://steamcommunity.com/id/me/inventoryhistory/?app[]=730";

struct CaseData {
  case: String,
  date: String,
  result: String,
}

#[tauri::command]
pub fn get_main(cookie: String) {
  std::thread::spawn(move || {
    let client = reqwest::blocking::Client::new();
    let res = client
      .get(BASE_URL)
      .header(COOKIE, &cookie)
      .send()
      .unwrap();

    let url = res.url().to_string();
    let body = res.text().unwrap();
    let username = get_username(&url);

    let mut cont = true;
    let mut time_frac = String::from("0");
    let mut time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let s_id = get_session_id(&body);
    let mut c = String::from("0");

    println!("{}", s_id);

    while cont {
      let ajax_url = format!(
        "https://steamcommunity.com/id/{}/inventoryhistory?ajax=1&cursor[time]={}&cursor[time_frac]={}&cursor[s]={}&sessionid={}&appid[]=730",
        username,
        time,
        time_frac,
        c,
        s_id
      );

      println!("{}", ajax_url);
      
      let res = client
        .get(&ajax_url)
        .header(COOKIE, &cookie)
        .send()
        .unwrap();

      let body = res.text().unwrap();
      let obj = match json::parse(&body) {
        Ok(o) => o,
        Err(e) => {
          println!("Error: {}", e);
          println!("Probably reached end of data.");
          break;
        }
      };

      if obj["num"].as_i32().unwrap() == 0 {
        cont = false;
        break;
      }

      if !obj["cursor"].is_empty() {
        c = obj["cursor"]["s"].as_str().unwrap().to_string();
        time = obj["cursor"]["time"].as_u64().unwrap() as u128;
        time_frac = obj["cursor"]["time_frac"].to_string();
      }
    }

    println!("Done!")
  });
}

pub fn get_session_id(html: &String) -> String {
  let re = Regex::new(r#"g_sessionID = "([^"]+)""#).unwrap();
  let session_id = re.captures(&html).unwrap().get(1).unwrap().as_str();
  session_id.to_string()
}

pub fn get_username(url: &String) -> String {
  let re = Regex::new(r#"https://steamcommunity.com/id/([^/]+)/"#).unwrap();
  let username = re.captures(&url).unwrap().get(1).unwrap().as_str();
  username.to_string()
}