use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  Crop { image: String, callback: String, x: u32, y: u32, width: u32, height: u32, error: String },
  Tint { image: String, callback: String, r: u8, g: u8, b: u8, error: String },
  SwapSkinColours { image: String, callback: String, new_colours: Vec<String>, error: String }
}
