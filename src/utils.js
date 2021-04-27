import { Base64 } from "js-base64";

// Crops an image; relies on calling the Tauri backend
export async function cropImage(imageBase64, x, y, width, height) {
  let croppedImage = window.__TAURI__.tauri
    .promisified({
      cmd: "crop",
      // Because the Rust base64 library doesn't like the
      // "data:image/png;base64," part; it's added back below
      image: imageBase64.substring(22),
      x: x,
      y: y,
      width: width,
      height: height,
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return croppedImage;
}

// Generic utility for flipping an image horizontally
// (for obtaining "left" sprites from "right" ones)
export async function mirrorImage(imageBase64) {
  let mirrored_image = window.__TAURI__.tauri.promisified({
    cmd: "mirrorImage",
    image: imageBase64.substring(22)
  })
  .then((response) => {
    return `data:image/png;base64,${response.value}`;
  })
  .catch((error) => {
    console.error(error)
  })

  return mirrored_image
}

// Image tinter; a similar implementation as with the crop function
export async function tintImage(imageBase64, r, g, b) {
  let tintedImage = window.__TAURI__.tauri
    .promisified({
      cmd: "tint",
      image: imageBase64.substring(22),
      r: r,
      g: g,
      b: b,
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return tintedImage;
}

// Swapping the skin colours; for body and arms sprites
export async function swapSkinColours(imageBase64, new_colours) {
  let swapped_image = window.__TAURI__.tauri
    .promisified({
      cmd: "swapSkinColours",
      image: imageBase64.substring(22),
      new_colours: new_colours,
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return swapped_image;
}

// Swapping the shoe colours; for the body sprite
export async function swapShoeColours(imageBase64, new_colours) {
  let swapped_image = window.__TAURI__.tauri
    .promisified({
      cmd: "swapShoeColours",
      image: imageBase64.substring(22),
      new_colours: new_colours,
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return swapped_image;
}

// Swapping the eye colour
export async function swapEyeColour(imageBase64, new_colour) {
  let swapped_image = window.__TAURI__.tauri
    .promisified({
      cmd: "swapEyeColour",
      image: imageBase64.substring(22),
      new_colour: new_colour,
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return swapped_image;
}

// Uses the Base64 library and some JS trickery to coerce an
// array of integers to become a base64-encoded image string
// that JS can interpret
export function generateSpriteDataUri(spriteBinaryArray) {
  return `data:image/png;base64,${Base64.fromUint8Array(
    new Uint8Array(spriteBinaryArray)
  )}`;
}

// Basically an easier way of reading a binary (i.e. not text) file
export async function fetchContent(path) {
  return await window.__TAURI__.fs.readBinaryFile(path);
}
