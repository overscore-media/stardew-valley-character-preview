import { Base64 } from "js-base64";

import { invoke } from '@tauri-apps/api/tauri'
import { readBinaryFile } from '@tauri-apps/api/fs'

// Crops an image; relies on calling the Tauri backend
export async function cropImage(imageBase64, x, y, width, height) {
  let croppedImage = invoke('crop_image',
    {
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
  let mirrored_image = invoke('mirror_image', 
  {
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
  let tintedImage = invoke('tint_image', 
  {
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
  let swapped_image = invoke('swap_skin_colours', 
    {
      image: imageBase64.substring(22),
      newColours: new_colours,
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
  let swapped_image = invoke('swap_shoe_colours', {
      image: imageBase64.substring(22),
      newColours: new_colours,
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
  let swapped_image = invoke('swap_eye_colour', 
  {
      image: imageBase64.substring(22),
      newColour: new_colour,
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
  return await readBinaryFile(path);
}
