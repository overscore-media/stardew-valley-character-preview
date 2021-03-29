<script>
  import txml from "txml";
  import BigNumber from "bignumber.js";
  import Grid from "gridjs-svelte";
  import { Base64 } from "js-base64";

  let saveFile = {};

  let boots,
    hat,
    pants,
    shirt = {};
  let hairstyleColor,
    newEyeColor = {};

  let name,
    eyeColor,
    gameVersion,
    hair,
    hairColor,
    gender,
    pantsColor,
    shirtColor,
    shoes,
    skin,
    skinColor;

  let data;
  let skinColoursSprite, farmerBaseSprite;

  async function getContentPath() {
    const contentPath = await window.__TAURI__.dialog.open({ directory: true });

    function generateSpriteDataUri(spriteBinaryArray) {
      return `data:image/png;base64,${Base64.fromUint8Array(
        new Uint8Array(spriteBinaryArray)
      )}`;
    }

    const skinColoursSpritesheet = await window.__TAURI__.fs.readBinaryFile(
      `${contentPath}/Characters/Farmer/skinColors.png`
    );

    skinColoursSprite = generateSpriteDataUri(skinColoursSpritesheet);
    
    console.log(`${contentPath}/Characters/Farmer/farmer_${gender === "Female" ? "girl_" : ""}base${hair === 0 ? "_bald" : ""}.png`);

    const farmerBase = await window.__TAURI__.fs.readBinaryFile(`${contentPath}/Characters/Farmer/farmer_${gender === "Female" ? "girl_" : ""}base${hair === 0 ? "_bald" : ""}.png`);

    farmerBaseSprite = generateSpriteDataUri(farmerBase);

  }

  async function getSaveFile() {
    const saveFilePath = await window.__TAURI__.dialog.open({
      defaultPath: "%AppData%\\StardewValley\\Saves\\",
    });
    const saveFileXML = await window.__TAURI__.fs.readTextFile(saveFilePath);

    saveFile = filterParsedXML(txml.parse(saveFileXML), "SaveGame");

    const player = saveFile.player;

    boots = player.boots;
    hat = player.hat;
    pants = player.pantsItem;
    shirt = player.shirtItem;

    hairstyleColor = player.hairstyleColor;
    newEyeColor = player.newEyeColor;

    name = player.name;
    gameVersion = player.gameVersion;
    hair = player.hair;
    hairColor = player.hairColor;
    eyeColor = player.eyeColor;
    gender = player.isMale ? "Male" : "Female";
    pantsColor = player.pantsColor;
    shirtColor = player.shirtColor;
    shoes = player.shoes;
    skin = player.skin;
    skinColor = player.skinColor;

    data = [
      { property: "Name", value: name },
      { property: "Eye Colour", value: eyeColor },
      { property: "Shirt Colour", value: shirtColor },
      { property: "Gender", value: gender },
      { property: "Hair Colour", value: hairColor },
      { property: "Skin", value: skin },
      { property: "Skin Colour", value: skinColor },
      { property: "Hair", value: hair },
      { property: "Shoes", value: shoes },
      { property: "Game Version", value: gameVersion },
    ];

    console.dir(hat);
    console.dir(pants);
    console.dir(shirt);
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
            if (
              Number(newValue).toString() === BigNumber(newValue).toString()
            ) {
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
</script>

<main>
  <h1 class="heading">Stardew Character Creator</h1>
  <button class="heading" on:click={getSaveFile}>Load Savefile</button>
  <button class="heading" on:click={getContentPath}
    >Load Unpacked Content Path</button
  >
  {#if skinColoursSprite}
    <img src={skinColoursSprite} alt="Skin Colours Sprite Sheet" />
  {/if}
  {#if farmerBaseSprite}
    <img src={farmerBaseSprite} alt="Skin Colours Sprite Sheet" />
  {/if}
  {#if data}
    <Grid {data} />
  {/if}
  <section class="character--preview">
    {#if "player" in saveFile}
      <hr />
      <h3>
        New Eye Colour: {`rgba(${newEyeColor.R}, ${newEyeColor.G}, ${newEyeColor.B}, ${newEyeColor.A})`}
      </h3>
      <h3>
        Hairstyle Colour: {`rgba(${hairstyleColor.R}, ${hairstyleColor.G}, ${hairstyleColor.B}, ${hairstyleColor.A})`}
      </h3>
      <h3>
        Pants Colour: {`rgba(${pantsColor.R}, ${pantsColor.G}, ${pantsColor.B}, ${pantsColor.A})`}
      </h3>
      <hr />
      <h3>Hat:</h3>
      <h4>Name: {hat.DisplayName}</h4>
      <h4>Category: {hat.category}</h4>
      <h4>Ignore Hairstyle Offset: {hat.ignoreHairstyleOffset}</h4>
      <h4>Skip Hair Draw: {hat.skipHairDraw}</h4>
      <hr />
      <h3>Shirt:</h3>
      <h4>Name: {shirt.DisplayName}</h4>
      <h4>
        Colour: {`rgba(${shirt.clothesColor.R}, ${shirt.clothesColor.G}, ${shirt.clothesColor.B}, ${shirt.clothesColor.A})`}
      </h4>
      <h4>Index in Tile Sheet: {shirt.indexInTileSheet}</h4>
      <h4>Index in Tile Sheet Female: {shirt.indexInTileSheetFemale}</h4>
      <h4>Parent Sheet Index: {shirt.parentSheetIndex}</h4>
      <hr />
      <h3>Pants:</h3>
      <h4>Name: {pants.DisplayName}</h4>
      <h4>Type: {pants.clothesType}</h4>
      <h4>
        Colour: {`rgba(${pants.clothesColor.R}, ${pants.clothesColor.G}, ${pants.clothesColor.B}, ${pants.clothesColor.A})`}
      </h4>
      <h4>Index in Tile Sheet: {pants.indexInTileSheet}</h4>
      <h4>Index in Tile Sheet Female: {pants.indexInTileSheetFemale}</h4>
      <h4>Parent Sheet Index: {pants.parentSheetIndex}</h4>
      <hr />
      <h3>Boots:</h3>
      <h4>Name: {boots.DisplayName}</h4>
      <h4>Colour Sheet Index: {boots.indexInColorSheet}</h4>
      <h4>Tile Sheet Index: {boots.indexInTileSheet}</h4>
      <hr />
    {/if}
    {#if "gameVersion" in saveFile}
      <h5>Game Version: {saveFile.gameVersion}</h5>
    {/if}
  </section>
</main>

<style>
  @import "https://cdn.jsdelivr.net/npm/gridjs/dist/theme/mermaid.min.css";

  .heading {
    display: flex;
    width: 100%;
    justify-content: center;
    text-align: center;
    font-size: 2rem;
    font-weight: bold;
  }

  .character--preview {
    display: flex;
    flex-flow: column nowrap;
    min-height: 70vh;
  }
</style>
