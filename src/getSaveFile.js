import txml from "txml";
import BigNumber from "bignumber.js";

export async function getSaveFile(current_path) {
  let saveFilePath
  
  if (!current_path) {
    saveFilePath = await window.__TAURI__.dialog.open();
  } else {
    saveFilePath = current_path
  }

  const saveFileXML = await window.__TAURI__.fs.readTextFile(saveFilePath);
  const saveFile = filterParsedXML(txml.parse(saveFileXML), "SaveGame");
  const player = saveFile.player;

  let shirtColor, pantsColor

  if (player.shirtColor) {
    shirtColor = `rgba(${player.shirtColor.R}, ${player.shirtColor.G}, ${player.shirtColor.B}, ${player.shirtColor.A / 255})`
  }

  if (player.pantsColor) {
    pantsColor = `rgba(${player.pantsColor.R}, ${player.pantsColor.G}, ${player.pantsColor.B}, ${player.pantsColor.A / 255})`
  }

  console.dir(player)

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
    eyeColor: `rgba(${player.newEyeColor.R}, ${player.newEyeColor.G}, ${player.newEyeColor.B}, ${player.newEyeColor.A / 255})`,
    hairColor: `rgba(${player.hairstyleColor.R}, ${player.hairstyleColor.G}, ${player.hairstyleColor.B}, ${player.hairstyleColor.A / 255})`,
    shoes: player.shoes,
    skin: player.skin,
    skinColor: player.skinColor,

    // Info
    name: player.name,
    gameVersion: player.gameVersion,
    gender: player.isMale ? "Male" : "Female",
    saveFilePath: saveFilePath
  };
}

function filterParsedXML(tag, attrib) {
  const matchingTag = tag.filter((tag) => {
    return tag.tagName == attrib;
  })[0];

  return internalParser(matchingTag.children);

  function internalParser(children) {
    let valuesObject = {};

    children.forEach((child) => {
      let newChild = {};
      newChild[child.tagName] = "";

      if (child.children.length === 1) {
        let newValue = child.children[0];
        if (newValue === "true") {
          newValue = true;
        } else if (newValue === "false") {
          newValue = false;
        } else if (!isNaN(newValue)) {
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
        newChild = internalParser(child.children);
      }

      valuesObject[child.tagName] = newChild;
    });
    return valuesObject;
  }
}
