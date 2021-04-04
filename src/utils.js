import { Base64 } from "js-base64";
import chroma from 'chroma-js'

export async function cropImage(imageBase64, x, y, width, height) {
  let croppedImage = window.__TAURI__.tauri
    .promisified({
      cmd: "crop",
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

export async function tintImage(imageBase64, r, g, b) {
  let tintedImage = window.__TAURI__.tauri
    .promisified({
      cmd: "tint",
      image: imageBase64.substring(22),
      r: r,
      g: g,
      b: b
    })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });

  return tintedImage;
}

export async function swapSkinColours(imageBase64, new_colours) {
  let swapped_image = window.__TAURI__.tauri.promisified({
    cmd: "swapSkinColours",
    image: imageBase64.substring(22),
    new_colours: new_colours
  })
    .then((response) => {
      return `data:image/png;base64,${response.value}`;
    })
    .catch((error) => {
      console.error(error);
    });


  return swapped_image
}

export function generateSpriteDataUri(spriteBinaryArray) {
  return `data:image/png;base64,${Base64.fromUint8Array(
    new Uint8Array(spriteBinaryArray)
  )}`;
}

export async function fetchContent(path) {
  return await window.__TAURI__.fs.readBinaryFile(path);
}