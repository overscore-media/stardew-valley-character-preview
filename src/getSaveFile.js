import txml from "txml";
import BigNumber from "bignumber.js";

import { open as openDialog } from '@tauri-apps/api/dialog'
import { readTextFile } from '@tauri-apps/api/fs'

export async function getSaveFile(current_path) {
  let saveFilePath;
  // Because for some reason the first dialog opened has an error
  let comError = false;

  if (!current_path) {
    saveFilePath = await openDialog().catch((err) => {
      if (err === "Could not initialize COM.") {
        comError = true;
      }
    });

    if (comError) {
      saveFilePath = await openDialog();
      comError = false;
    }
  } else {
    saveFilePath = current_path;
  }

  // Read the file from the path the dialog (or current_path) gave
  const saveFileXML = await readTextFile(saveFilePath);
  // Parse out the savefile; look for the "SaveGame" section
  const saveFile = filterParsedXML(txml.parse(saveFileXML), "SaveGame");
  // Get the "player" attribute from the savefile
  const player = saveFile.player;

  let shirtColor, pantsColor;

  // Put the shirt colour and pants colour into nice RGBA strings if they exist
  if (player.shirtColor) {
    shirtColor = `rgba(${player.shirtColor.R}, ${player.shirtColor.G}, ${
      player.shirtColor.B
    }, ${player.shirtColor.A / 255})`;
  }

  if (player.pantsColor) {
    pantsColor = `rgba(${player.pantsColor.R}, ${player.pantsColor.G}, ${
      player.pantsColor.B
    }, ${player.pantsColor.A / 255})`;
  }

  return {
    // Cosmetic Items
    accessory: player.accessory,
    hat: player.hat,
    shirtItem: player.shirtItem,
    shirt: player.shirt,
    shirtColor: shirtColor,
    pantsItem: player.pantsItem,
    pants: player.pants,
    pantsColor: pantsColor,
    boots: player.boots,

    // Appearance
    hair: player.hair,
    eyeColor: `rgba(${player.newEyeColor.R}, ${player.newEyeColor.G}, ${
      player.newEyeColor.B
    }, ${player.newEyeColor.A / 255})`,
    hairColor: `rgba(${player.hairstyleColor.R}, ${player.hairstyleColor.G}, ${
      player.hairstyleColor.B
    }, ${player.hairstyleColor.A / 255})`,
    shoes: player.shoes,
    skin: player.skin,
    skinColor: player.skinColor,

    // Info
    name: player.name,
    gameVersion: player.gameVersion,
    gender: player.isMale ? "Male" : "Female",
    saveFilePath: saveFilePath,
  };
}

// Yeah, this is a doozy
function filterParsedXML(tag, attrib) {
  const matchingTag = tag.filter((tag) => {
    return tag.tagName == attrib;
  })[0];

  return internalParser(matchingTag.children);

  // This doesn't even really make sense anymore, but it does work
  // It deals with the fact that txml creates a weird type of object for the data from the
  // XML savefile, but it's quick and there aren't really many better options
  // Also this is required to do some... type coercion for lack of a better word
  function internalParser(children) {
    let valuesObject = {};

    children.forEach((child) => {
      let newChild = {};
      newChild[child.tagName] = "";

      if (child.children.length === 1) {
        let newValue = child.children[0];

        // Basically some type coercion but a janky custom implementation
        if (newValue === "true") {
          // There's probably a better way to do this...
          newValue = true;
        } else if (newValue === "false") {
          newValue = false;
        } else if (!isNaN(newValue)) {
          // Only converts numbers whose precision doesn't exceed that of JS's
          // BigNumber checks that; otherwise it'll stay a string
          if (Number(newValue).toString() === BigNumber(newValue).toString()) {
            newValue = Number(newValue);
          }
        } else {
          if (typeof newValue !== "string") {
            newValue = internalParser(child.children);
          }
        }
        newChild = newValue;
      } else {
        // There's some recursion going on, and you'll see it above too
        newChild = internalParser(child.children);
      }

      valuesObject[child.tagName] = newChild;
    });
    return valuesObject;
  }
}
