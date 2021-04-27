use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // Custom commands that can be called from JS
  Crop { image: String, callback: String, x: u32, y: u32, width: u32, height: u32, error: String },
  Tint { image: String, callback: String, r: u8, g: u8, b: u8, error: String },
  MirrorImage { image: String, callback: String, error: String },
  SwapSkinColours { image: String, callback: String, new_colours: Vec<String>, error: String },
  SwapShoeColours { image: String, callback: String, new_colours: Vec<String>, error: String },
  SwapEyeColour { image: String, callback: String, new_colour: String, error: String }
}
