#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use image::{DynamicImage, LumaA, Pixel, Rgba};

use imageproc::map::map_colors;

use serde::Serialize;

#[derive(Serialize)]
struct CropResponse {
  value: String,
  callback: String,
  error: String
}

#[derive(Serialize)]
struct TintResponse {
  value: String,
  callback: String,
  error: String
}


#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> std::error::Error for CommandError<'a> {}

pub fn tint(gray: LumaA<u8>, color: Rgba<u8>) -> Rgba<u8> {
  Rgba([((gray[0] as u32 * color.channels()[0] as u32) / 255u32) as u8, ((gray[0] as u32 * color.channels()[3] as u32) / 255u32) as u8, ((gray[0] as u32 * color.channels()[2] as u32) / 255u32) as u8, (std::cmp::min(gray[0], 1) * 255u8) as u8])
}

mod cmd;

fn main() {
  
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {

            Crop { image, x, y, width, height, callback, error } => tauri::execute_promise(_webview, move || {
              let image_string = String::from(&image);
              let image_decoded = base64::decode(&image_string).unwrap();
              let mut image_loaded = image::load_from_memory(&image_decoded).unwrap();
              let image_cropped = image_loaded.crop(x, y, width, height);

              let mut write_buffer = vec![];
              image_cropped.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
              let image_encoded = base64::encode(&write_buffer);


              if image_encoded.len() > 0 {
              let image_processed = &image_encoded;

              let response = CropResponse {
                value: image_processed.to_string(),
                callback: "callback".to_string(),
                error: "error".to_string()
              };

              Ok(response)
            } else {
              Err(CommandError::new("could not read image").into())
            }

            },
            callback,
            error,
          ),

          Tint { image,r, g, b, callback, error } => tauri::execute_promise(_webview, move || {
            let image_string = String::from(&image);
            let image_decoded = base64::decode(&image_string).unwrap();
            let image_loaded = image::load_from_memory(&image_decoded).unwrap();
            let image_luma = image_loaded.to_luma_alpha8();

            let image_tinted = map_colors(&image_luma, |pix| tint(pix, Rgba([r, g, b, 255u8])));
            let image_tinted_dynamic = DynamicImage::ImageRgba8(image_tinted);

            let mut write_buffer = vec![];
            image_tinted_dynamic.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
            let image_encoded = base64::encode(&write_buffer);


            if image_encoded.len() > 0 {
            let image_processed = &image_encoded;

            let response = CropResponse {
              value: image_processed.to_string(),
              callback: "callback".to_string(),
              error: "error".to_string()
            };

            Ok(response)
          } else {
            Err(CommandError::new("could not read image").into())
          }

          },
          callback,
          error,
        ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
