#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// External libraries
use image::{DynamicImage, Pixel, Rgba};
use imageproc::map::map_colors;
use css_color_parser::Color as CssColor;
use serde::Serialize;

#[derive(Serialize)]
struct Response<'a> {
  value: std::string::String,
  message: &'a str,
}

#[derive(Debug, Clone, Serialize)]
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

// The tint function
// You may recognize this as a "Multiply Blend"
pub fn tint(pixel: Rgba<u8>, color: Rgba<u8>) -> Rgba<u8> {
  Rgba([((pixel.channels()[0] as u32 * color.channels()[0] as u32) / 255u32) as u8, ((pixel.channels()[1] as u32 * color.channels()[1] as u32) / 255u32) as u8, ((pixel.channels()[2] as u32 * color.channels()[2] as u32) / 255u32) as u8, pixel.channels()[3] as u8])
}

// Changes the default skin colours in an image (body/arms sprite) to those from another colour spritesheet
pub fn skin_colour_swap(pixel: Rgba<u8>, colour_1: CssColor, colour_2: CssColor, colour_3: CssColor) -> Rgba<u8> {
  let default_1 = Rgba([249u8, 174u8, 137u8, 255u8]);
  let default_2 = Rgba([224u8, 107u8, 101u8, 255u8]);
  let default_3 = Rgba([107u8, 0u8, 58u8, 255u8]);

  if pixel.eq(&default_1) {
    Rgba([colour_1.r, colour_1.g, colour_1.b, (colour_1.a * 255f32) as u8])
  } else if pixel.eq(&default_2) {
    Rgba([colour_2.r, colour_2.g, colour_2.b, (colour_2.a * 255f32) as u8])
  } else if pixel.eq(&default_3) {
    Rgba([colour_3.r, colour_3.g, colour_3.b, (colour_3.a * 255f32) as u8])
  } else {
    pixel
  }
}

// Very similar to above but for shoe colours
pub fn shoe_colour_swap(pixel: Rgba<u8>, colour_1: CssColor, colour_2: CssColor, colour_3: CssColor, colour_4: CssColor) -> Rgba<u8> {
  let default_1 = Rgba([173u8, 71u8, 27u8, 255u8]);
  let default_2 = Rgba([119u8, 41u8, 26u8, 255u8]);
  let default_3 = Rgba([91u8, 31u8, 36u8, 255u8]);
  let default_4 = Rgba([61u8, 17u8, 35u8, 255u8]);

  if pixel.eq(&default_1) {
    Rgba([colour_1.r, colour_1.g, colour_1.b, (colour_1.a * 255f32) as u8])
  } else if pixel.eq(&default_2) {
    Rgba([colour_2.r, colour_2.g, colour_2.b, (colour_2.a * 255f32) as u8])
  } else if pixel.eq(&default_3) {
    Rgba([colour_3.r, colour_3.g, colour_3.b, (colour_3.a * 255f32) as u8])
  } else if pixel.eq(&default_4) {
    Rgba([colour_4.r, colour_4.g, colour_4.b, (colour_4.a * 255f32) as u8])
  } else {
    pixel
  }
}

// Similar to above, but for just one colour
pub fn eye_colour_swap(pixel: Rgba<u8>, colour: CssColor) -> Rgba<u8> {
  let default = Rgba([104u8, 43u8, 15u8, 255u8]);

  if pixel.eq(&default) {
    Rgba([colour.r, colour.g, colour.b, (colour.a * 255f32) as u8])
  } else {
    pixel
  }
}

#[tauri::command]
fn crop_image(image: String, x: u32, y: u32, width: u32, height: u32) -> Result<Response<'static>, CommandError<'static>> {
  // Turn the base64-encoded image string from JS into a vector of integers
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();

  // Load the u8 vector above into a DynamicImage-type variable
  let mut image_loaded = image::load_from_memory(&image_decoded).unwrap();

  // Call the image library's crop function on the image
  let image_cropped = image_loaded.crop(x, y, width, height);

  // Write the cropped DynamicImage to a buffer, somehow, and convert it back to a base64 string
  let mut write_buffer = vec![];
  image_cropped.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // If the encoded string exists, return it; otherwise, throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;

    let response = Response {
      value: image_processed.to_string(),
      message: "Image cropped successfully"
    };

    Ok(response)
  } else {
    Err(CommandError::new("Could not crop image").into())
  }
}

#[tauri::command]
fn tint_image(image: String, r: u8, g: u8, b:u8) -> Result<Response<'static>, CommandError<'static>> {
  // Similar as with the Crop function, but this time we're converting the DynamicImage into an Rgba<u8> ImageBuffer
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();
  let image_loaded = image::load_from_memory(&image_decoded).unwrap();
  let image_rgba = image_loaded.to_rgba8();

  // For every pixel, run it through the tinting function using the R, G, and B values the function was called with
  let image_tinted = map_colors(&image_rgba, |pix| tint(pix, Rgba([r, g, b, 255u8])));
  let image_tinted_dynamic = DynamicImage::ImageRgba8(image_tinted);

  // Write the Dynamic (tinted) image to a buffer, and base64 encode it
  let mut write_buffer = vec![];
  image_tinted_dynamic.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // Send it off or throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;
    let response = Response {
      value: image_processed.to_string(),
      message: "Image tinted successfully"
    };

    Ok(response)
  } else {
    Err(CommandError::new("Could not tint image").into())
  }
}

#[tauri::command]
fn mirror_image(image: String) -> Result<Response<'static>, CommandError<'static>> {
  // Turn the base64-encoded image string from JS into a vector of integers
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();

  // Load the u8 vector above into a DynamicImage-type variable
  let mut image_loaded = image::load_from_memory(&image_decoded).unwrap();

  // Call the image library's flip_horizontal function on the image
  image::imageops::flip_horizontal_in_place(&mut image_loaded);

  // Write the mirrored DynamicImage to a buffer, somehow, and convert it back to a base64 string
  let mut write_buffer = vec![];
  image_loaded.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // If the encoded string exists, return it; otherwise, throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;
    let response = Response {
      value: image_processed.to_string(),
      message: "Image mirrored successfully"
    };

    Ok(response)
  } else {
    Err(CommandError::new("Could not mirror image").into())
  }
}

#[tauri::command]
fn swap_skin_colours(image: String, new_colours: Vec<String>) -> Result<Response<'static>, CommandError<'static>> {
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();
  let image_loaded = image::load_from_memory(&image_decoded).unwrap();
  let image_rgba = image_loaded.to_rgba8();

  // Turning the colours vector into CssColor's
  let colour_1 = new_colours[2].parse::<CssColor>().unwrap();
  let colour_2 = new_colours[1].parse::<CssColor>().unwrap();
  let colour_3 = new_colours[0].parse::<CssColor>().unwrap();

  // Run the skin_colour_swap function on each pixel then convert the result to a DynamicImage
  let image_colour_swapped = map_colors(&image_rgba, |pix| skin_colour_swap(pix, colour_1, colour_2, colour_3) );
  let image_swapped_dynamic = DynamicImage::ImageRgba8(image_colour_swapped);

  // Write the new Dynamic image to a buffer, and base64 encode it
  let mut write_buffer = vec![];
  image_swapped_dynamic.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // Send it off or throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;

    let response = Response {
      value: image_processed.to_string(),
      message: "Skin colours swapped successfully"
    };

    Ok(response)
  } else {
    Err(CommandError::new("Could not swap skin colours").into())
  }
}

#[tauri::command]
fn swap_shoe_colours(image: String, new_colours: Vec<String>) -> Result<Response<'static>, CommandError<'static>> {
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();
  let image_loaded = image::load_from_memory(&image_decoded).unwrap();
  let image_rgba = image_loaded.to_rgba8();

  // Turning the colours vector into CssColor's
  let colour_1 = new_colours[3].parse::<CssColor>().unwrap();
  let colour_2 = new_colours[2].parse::<CssColor>().unwrap();
  let colour_3 = new_colours[1].parse::<CssColor>().unwrap();
  let colour_4 = new_colours[0].parse::<CssColor>().unwrap();

  // Run the skin_colour_swap function on each pixel then convert the result to a DynamicImage
  let image_colour_swapped = map_colors(&image_rgba, |pix| shoe_colour_swap(pix, colour_1, colour_2, colour_3, colour_4));
  let image_swapped_dynamic = DynamicImage::ImageRgba8(image_colour_swapped);

  // Write the new Dynamic image to a buffer, and base64 encode it
  let mut write_buffer = vec![];
  image_swapped_dynamic.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // Send it off or throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;

    let response = Response {
      value: image_processed.to_string(),
      message: "Shoe colours swapped successfully"
    };

    Ok(response)
  } else {
    Err(CommandError::new("Could not swap shoe colours").into())
  }
}

#[tauri::command]
fn swap_eye_colour(image: String, new_colour: String) -> Result<Response<'static>, CommandError<'static>> {
  let image_string = String::from(&image);
  let image_decoded = base64::decode(&image_string).unwrap();
  let image_loaded = image::load_from_memory(&image_decoded).unwrap();
  let image_rgba = image_loaded.to_rgba8();

  // Turning the colour vector into a CssColor
  let colour = new_colour.parse::<CssColor>().unwrap();

  // Run the skin_colour_swap function on each pixel then convert the result to a DynamicImage
  let image_colour_swapped = map_colors(&image_rgba, |pix| eye_colour_swap(pix, colour));
  let image_swapped_dynamic = DynamicImage::ImageRgba8(image_colour_swapped);

  // Write the new Dynamic image to a buffer, and base64 encode it
  let mut write_buffer = vec![];
  image_swapped_dynamic.write_to(&mut write_buffer, image::ImageOutputFormat::Png).unwrap();
  let image_encoded = base64::encode(&write_buffer);

  // Send it off or throw an error
  if image_encoded.len() > 0 {
    let image_processed = &image_encoded;

    let response = Response {
      value: image_processed.to_string(),
      message: "Eye colour swapped successfully"
    };
    Ok(response)
  } else {
    Err(CommandError::new("Could not swap eye colour").into())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![crop_image, tint_image, mirror_image, swap_skin_colours, swap_shoe_colours, swap_eye_colour])
    .run(tauri::generate_context!())
    .expect("An error occurred while running the application");
}