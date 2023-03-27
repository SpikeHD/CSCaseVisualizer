use reqwest::{self, header::COOKIE};
use tauri::regex::Regex;
use std::{time::{SystemTime, UNIX_EPOCH}};
use scraper;
use json;

static BASE_URL: &str = "https://steamcommunity.com/id/me/inventoryhistory/?app[]=730";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CaseData {
  case: String,
  case_img: String,
  date: String,
  result: String,
  result_img: String,
  rarity: RarityData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RarityData {
  condition: String,
  rarity: String,
}

#[tauri::command]
pub fn get_main(window: tauri::Window, cookie: String) {
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

    if username.is_empty() {
      window.emit("error", "Currently being ratelimited. Wait a few minutes and try again.").unwrap();
      return;
    }

    let mut time_frac = String::from("0");
    let mut time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let s_id = get_session_id(&body);
    let mut c = String::from("0");

    // The full data list
    let mut list: Vec<CaseData> = Vec::new();
    let mut page_count = 0;

    println!("{}", s_id);

    loop {
      // DEBUG
      if page_count > 4 {
        break;
      }

      let ajax_url = format!(
        "https://steamcommunity.com/id/{}/inventoryhistory?ajax=1&cursor[time]={}&cursor[time_frac]={}&cursor[s]={}&sessionid={}&appid[]=730",
        username,
        time,
        time_frac,
        c,
        s_id
      );
      
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
        break;
      }

      if !obj["cursor"].is_empty() {
        c = obj["cursor"]["s"].as_str().unwrap().to_string();
        time = obj["cursor"]["time"].as_u64().unwrap() as u128;
        time_frac = obj["cursor"]["time_frac"].to_string();
      }

      let mut inner_list = scrape_page(obj);

      list.append(&mut inner_list);

      page_count += 1;

      window.emit("page_process", page_count).unwrap();
    }

    println!("Done! List size: {}", list.len());

    window.emit("finish_process", &list).unwrap();
  });
}

pub fn scrape_page(json: json::JsonValue) -> Vec<CaseData> {
  let mut list: Vec<CaseData> = Vec::new();

  let html = json["html"].as_str().unwrap().to_string();
  let document = scraper::Html::parse_document(&html);

  let history_sel = &scraper::Selector::parse(".tradehistoryrow").unwrap();
  let history = document.select(history_sel);

  for entry in history {
    let reason_sel = &scraper::Selector::parse(".tradehistory_event_description").unwrap();
    let reason = entry.select(reason_sel).next().unwrap().text().collect::<Vec<_>>().join(" ");

    // Must make sure it's a case
    if !reason.contains("Unlocked a container") {
      continue;
    }

    let name_sel = &scraper::Selector::parse(".history_item").unwrap();
    let mut name_res = entry.select(name_sel);
    let case_name = name_res.next().unwrap().text().collect::<Vec<_>>().join(" ");

    // The second name usually isn't the item name, but sometimes it is, so check if there are 3 matches
    let mut count = 0;
    for _ in name_res.clone() {
      count += 1;
    }

    if count > 1 {
      name_res.next();
    }

    // Get instance and class id
    let name_inst = name_res.next().unwrap();
    let item_inst_id = name_inst.value().attr("data-instanceid").unwrap().to_string();
    let item_class_id = name_inst.value().attr("data-classid").unwrap().to_string();

    let item_rarity_data = get_rarity_from_data(json["descriptions"].clone(), item_inst_id, item_class_id);

    let item_name = name_inst.text().collect::<Vec<_>>().join(" ");

    // Images
    let img_sel = &scraper::Selector::parse(".tradehistory_received_item_img").unwrap();
    let mut img_res = entry.select(img_sel);
    let case_img = img_res.next().unwrap().value().attr("src").unwrap().to_string();

    // The second img usually isn't the item img, but sometimes it is, so check if there are 3 matches
    let mut count = 0;
    for _ in img_res.clone() {
      count += 1;
    }

    if count > 1 {
      img_res.next();
    }

    let item_img = img_res.next().unwrap().value().attr("src").unwrap().to_string();

    // Date
    let date_sel = &scraper::Selector::parse(".tradehistory_date").unwrap();
    let date = entry.select(date_sel).next().unwrap().text().collect::<Vec<_>>().join(" ");

    let case_data = CaseData {
      case: case_name,
      case_img,
      date,
      result: item_name,
      result_img: item_img,
      rarity: item_rarity_data
    };

    list.push(case_data);

    std::thread::sleep(std::time::Duration::from_millis(2000));
  }

  list
}

fn get_rarity_from_data(descriptions: json::JsonValue, instanceid: String, classid: String) -> RarityData {
  let key = format!("{}_{}", classid, instanceid);
  let tags = &descriptions["730"][&key]["tags"];

  let mut cond_idx = 5;
  let mut rare_idx = 4;

  // Count tag length
  let mut count = 0;
  for _ in tags.clone().members() {
    count += 1;
  }

  if count < 5 {
    cond_idx = count;
    rare_idx = count - 1;
  }

  RarityData {
    condition: tags[cond_idx]["name"].as_str().unwrap_or("").to_string(),
    rarity: tags[rare_idx]["internal_name"].as_str().unwrap_or("").to_string(),
  }
}

pub fn get_session_id(html: &String) -> String {
  let re = Regex::new(r#"g_sessionID = "([^"]+)""#).unwrap();
  let session_id = re.captures(&html).unwrap().get(1).unwrap().as_str();
  session_id.to_string()
}

pub fn get_username(url: &String) -> String {
  let re = Regex::new(r#"https://steamcommunity.com/id/([^/]+)/"#).unwrap();
  let username = match re.captures(&url).unwrap().get(1) {
    Some(u) => u.as_str(),
    None => return "".to_string(),
  };
  username.to_string()
}