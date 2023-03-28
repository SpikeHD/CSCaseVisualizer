use crate::inventory::CaseData;

#[tauri::command]
pub fn list_dumps() -> Vec<std::string::String> {
  let mut list: Vec<String> = Vec::new();
  let paths = std::fs::read_dir("dumps").unwrap();
  for path in paths {
    let path = path.unwrap().path();
    let path = path.to_str().unwrap().to_string();
    list.push(path);
  }

  list
}

#[tauri::command]
pub fn get_dump(path: String) -> Vec<CaseData> {
  let file = std::fs::read_to_string(path).unwrap();
  let list: Vec<CaseData> = serde_json::from_str(&file).unwrap();

  list
}