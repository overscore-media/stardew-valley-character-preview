<script>
  import { createEventDispatcher } from "svelte";

  import { getSaveFile } from "./getSaveFile";
  import {
    cropImage,
    tintImage,
    fetchContent,
    generateSpriteDataUri,
  } from "./utils";
  import chroma from "chroma-js";
  import {
    Grid,
    Row,
    Column,
    TextInput,
    Button,
    ProgressIndicator,
    InlineNotification,
    ProgressStep,
    Loading,
  } from "carbon-components-svelte";

  import LicenseThirdParty32 from "carbon-icons-svelte/lib/LicenseThirdParty32";
  import Folder32 from "carbon-icons-svelte/lib/Folder32";
  import WatsonHealthHangingProtocol32 from "carbon-icons-svelte/lib/WatsonHealthHangingProtocol32";

  let player;

  let sprites = {};

  let custom_hat = false;
  let custom_hair = false;
  let custom_shirt = false;
  let custom_pants = false;

  let custom_hat_path, custom_hair_path, custom_shirt_path, custom_pants_path;

  let player_shoecolour,
    player_skincolour,
    player_eyes,
    player_pants,
    player_hat,
    player_hair,
    player_body,
    player_accessory,
    player_arms,
    player_shirt;

  let player_canvas;

  let progress_bar_index = 0;

  let content_path = "";

  let drawing_player = false;
  let player_drawn = false;

  const dispatcher = createEventDispatcher();

  async function loadSave() {
    player = await getSaveFile();
    if (player) {
      console.dir(player);
      progress_bar_index = 1;
    }
  }

  async function getContent() {
    content_path = await window.__TAURI__.dialog.open({ directory: true });

    async function addSprite(spriteName) {
      const spriteToAdd = await fetchContent(
        `${content_path}/Characters/Farmer/${spriteName}.png`
      );

      sprites[spriteName] = generateSpriteDataUri(spriteToAdd);
    }

    if (content_path) {
      [
        "skinColors",
        "shoeColors",
        "accessories",
        "hats",
        "shirts",
        "pants",
        "hairstyles",
      ].forEach((sprite) => {
        addSprite(sprite);
      });

      const farmerBase = await fetchContent(
        `${content_path}/Characters/Farmer/farmer_${
          player.gender === "Female" ? "girl_" : ""
        }base${player.hair === 0 ? "_bald" : ""}.png`
      );

      sprites.farmerBase = generateSpriteDataUri(farmerBase);
      progress_bar_index = 1;
    }
  }

  async function updateSprites() {
    player_shoecolour = await cropImage(
      sprites.shoeColors,
      0,
      player.shoes,
      4,
      1
    );

    player_skincolour = await cropImage(
      sprites.skinColors,
      0,
      player.skinColor,
      3,
      1
    );

    await updateArms();
    await updateBody();
    await updateHats();
    await updateAccessories();
    await updateHair();
    await updateShirt();
    await updatePants();
  }

  async function updateHair() {
    let hairRow = 1;
    let hairCol = 1;
    let hairSprite = sprites.hairstyles;

    if (!custom_hair) {
      hairRow = Math.ceil(player.hair / 7);
      hairCol = player.hair - (hairRow - 1) * 7;
    } else {
      hairSprite = custom_hair;
    }

    const player_hair_sprite = await cropImage(
      hairSprite,
      (hairCol - 1) * 16,
      hairRow * 96,
      16,
      32
    );

    const hairTint = chroma(player.hairColor).rgba();

    const player_hair_tinted = await tintImage(
      player_hair_sprite,
      hairTint[0],
      hairTint[1],
      hairTint[2]
    );

    player_hair = new Image();
    player_hair.src = player_hair_tinted;
  }

  async function updateHats() {
    let hatIndex = player.hat.which;
    let hatSprite = sprites.hats;

    if (custom_hat) {
      hatIndex = 1;
      hatSprite = custom_hat;
    }

    if (hatIndex) {
      let hatRow = 1;
      let hatCol = 1;

      if (!custom_hat) {
        hatRow = Math.ceil((hatIndex + 1) / 12);
        hatCol = hatIndex - 12 * (hatRow - 1) + 1;
      }

      const player_hat_sprite = await cropImage(
        hatSprite,
        (hatCol - 1) * 20,
        (hatRow - 1) * 80,
        20,
        20
      );

      player_hat = new Image();
      player_hat.src = player_hat_sprite;
    } else {
      player_hat = new Image();
    }
  }

  async function updateArms() {
    const player_arms_sprite = await cropImage(
      sprites.farmerBase,
      96,
      0,
      16,
      32
    );

    player_arms = new Image();
    player_arms.src = player_arms_sprite;
  }

  async function updateBody() {
    const player_body_sprite = await cropImage(
      sprites.farmerBase,
      0,
      0,
      16,
      32
    );

    player_body = new Image();
    player_body.src = player_body_sprite;
  }

  async function updateAccessories() {
    let accessoryIndex;

    if (player.accessory > 0) {
      accessoryIndex = player.accessory;

      const accessoryRow = Math.ceil(accessoryIndex / 8);
      const accessoryCol = accessoryIndex - Math.abs((accessoryRow - 1) * 8);

      const player_accessory_sprite = await cropImage(
        sprites.accessories,
        (accessoryCol - 1) * 16,
        (accessoryRow - 1) * 32,
        16,
        16
      );

      player_accessory = new Image();
      player_accessory.src = player_accessory_sprite;
    } else {
      player_accessory = new Image();
    }
  }

  async function updatePants() {
    let pants_sprite_sheet = sprites.pants;
    let player_pants_sprite;

    let pantsIndex = 0;

    if (player.pantsItem) {
      pantsIndex = player.pantsItem.indexInTileSheet;
    } else if (player.pants > 0) {
      pantsIndex = player.pants;
    }

    if (custom_pants) {
      pants_sprite_sheet = custom_pants;
      pantsIndex = 0;
    }

    let pantsRow = Math.ceil((pantsIndex + 1) / 10);
    let pantsCol = pantsIndex + 1 - 10 * (pantsRow - 1);

    player_pants_sprite = await cropImage(
      pants_sprite_sheet,
      (pantsCol - 1) * 192,
      (pantsRow - 1) * 688,
      16,
      32
    );

    if (player.pantsColor) {
      let pantsTint = chroma(player.pantsColor).rgba();

      player_pants_sprite = await tintImage(
        player_pants_sprite,
        pantsTint[0],
        pantsTint[1],
        pantsTint[2]
      );
    }

    player_pants = new Image();
    player_pants.src = player_pants_sprite;
  }

  async function updateShirt() {
    let player_shirt_sprite;

    async function fetchShirt(shirtIndex, shirtIndexFemale) {
      const shirts = new Image();
      shirts.src = sprites.shirts;

      let x,
        y = 0;
      let row = 0;
      let column = 0;
      let index = shirtIndex;

      const canvas = new OffscreenCanvas(shirts.width, shirts.height);
      const context = canvas.getContext("2d");
      context.drawImage(shirts, 0, 0, shirts.width, shirts.height);

      if (shirtIndexFemale !== -1 && player.gender === "Female") {
        index = shirtIndexFemale;
      }

      row = Math.ceil((shirtIndex + 1) / 16);
      column = shirtIndex + 1 - Math.abs((row - 1) * 16);

      x = (column - 1) * 8;
      y = (row - 1) * 32;

      if (shirtIndex < 128) {
        return await cropImage(sprites.shirts, x, y, 8, 8);
      } else {
        if (
          context
            .getImageData(x, y, 8, 32)
            .data.reduce((prev, val) => prev + val)
        ) {
          return await cropImage(sprites.shirts, x, y, 8, 8);
        } else {
          return await cropImage(sprites.shirts, x + 128, y, 8, 8);
        }
      }
    }

    if (custom_shirt) {
      player_shirt_sprite = await cropImage(custom_shirt, 0, 0, 8, 8);
    } else if (player.shirtItem) {
      player_shirt_sprite = await fetchShirt(
        player.shirtItem.indexInTileSheet,
        player.shirtItem.indexInTileSheetFemale
      );
    } else {
      // Default Female shirt
      player_shirt_sprite = await fetchShirt(130);
    }

    let shirtTint;

    if (player.shirtColor) {
      shirtTint = chroma(player.shirtColor).rgba();
      player_shirt_sprite = await tintImage(
        player_hair_sprite,
        shirtTint[0],
        shirtTint[1],
        shirtTint[2]
      );
    }

    player_shirt = new Image();
    player_shirt.src = player_shirt_sprite;
  }

  async function uploadSprite(sprite_type) {
    const spritePath = await window.__TAURI__.dialog.open();
    const spriteData = await fetchContent(spritePath);

    if (sprite_type === "hair") {
      custom_hair_path = spritePath;
      custom_hair = generateSpriteDataUri(spriteData);
    } else if (sprite_type === "hat") {
      custom_hat_path = spritePath;
      custom_hat = generateSpriteDataUri(spriteData);
    } else if (sprite_type === "shirt") {
      custom_shirt_path = spritePath;
      custom_shirt = generateSpriteDataUri(spriteData);
    } else if (sprite_type === "pants") {
      custom_pants_path = spritePath;
      custom_pants = generateSpriteDataUri(spriteData);
    }
  }

  async function drawCharacter() {
    drawing_player = true;
    player_drawn = false;

    await updateSprites();

    const ctx = player_canvas.getContext("2d");

    const offsetX = 4;
    const offsetY = 1;

    ctx.drawImage(player_body, 0 + offsetX, 0 + offsetY);

    ctx.fillStyle = player.eyeColor;
    ctx.fillRect(6 + offsetX, 12 + offsetY, 1, 2);
    ctx.fillRect(9 + offsetX, 12 + offsetY, 1, 2);

    ctx.drawImage(player_pants, 0 + offsetX, 0 + offsetY);
    ctx.drawImage(player_shirt, 4 + offsetX, 16 + offsetY);
    ctx.drawImage(player_accessory, 0 + offsetX, 3 + offsetY);
    ctx.drawImage(player_hair, 0 + offsetX, 0 + offsetY);
    ctx.drawImage(player_hat, -2 + offsetX, -1 + offsetY);
    ctx.drawImage(player_arms, 0 + offsetX, 0 + offsetY);

    drawing_player = false;
    player_drawn = true;
  }

  $: document.documentElement.setAttribute("theme", "g100");
</script>

<main>
  <Grid fullWidth class="player--wrapper">
    <Row>
      <Column>
        <section class="input--wrapper">
          <div class="inputs">
            <Row>
              <TextInput
                size="sm"
                disabled
                value={player ? player.saveFilePath : ""}
                labelText="Savefile Path:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={LicenseThirdParty32}
                  on:click={loadSave}
                />
              </div>
            </Row>
            <Row>
              <TextInput
                size="sm"
                disabled
                value={content_path ? content_path : ""}
                labelText="Unpacked Content Folder:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={Folder32}
                  on:click={getContent}
                />
              </div>
            </Row>
            <ProgressIndicator
              currentIndex={progress_bar_index}
              spaceEqually
              preventChangeOnClick
            >
              <ProgressStep
                disabled={player
                  ? "saveFilePath" in player
                    ? false
                    : true
                  : true}
                complete={player ? "saveFilePath" in player : false}
                label="Load Savefile"
              />
              <ProgressStep
                disabled={!content_path}
                complete={content_path}
                label="Load Content"
              />
            </ProgressIndicator>
            <div class="center">
              <Button
                disabled={progress_bar_index < 1 || !content_path}
                on:click={drawCharacter}>Draw Character</Button
              >
            </div>
            <Row>
              <TextInput
                size="sm"
                disabled
                value={custom_hair_path ? custom_hair_path : ""}
                labelText="Custom Hair:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={WatsonHealthHangingProtocol32}
                  on:click={() => uploadSprite("hair")}
                />
              </div>
            </Row>
            <Row>
              <TextInput
                size="sm"
                disabled
                value={custom_hat_path ? custom_hat_path : ""}
                labelText="Custom Hat:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={WatsonHealthHangingProtocol32}
                  on:click={() => uploadSprite("hat")}
                />
              </div>
            </Row>
            <Row>
              <TextInput
                size="sm"
                disabled
                value={custom_shirt_path ? custom_shirt_path : ""}
                labelText="Custom Shirt:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={WatsonHealthHangingProtocol32}
                  on:click={() => uploadSprite("shirt")}
                />
              </div>
            </Row>
            <Row>
              <TextInput
                size="sm"
                disabled
                value={custom_pants_path ? custom_pants_path : ""}
                labelText="Custom Pants:"
              />
              <div class="button">
                <Button
                  size="small"
                  iconDescription="Browse"
                  icon={WatsonHealthHangingProtocol32}
                  on:click={() => uploadSprite("pants")}
                />
              </div>
            </Row>
          </div>
        </section>
      </Column>
      <Column>
        <section class="player--wrapper">
          <canvas
            class={player_drawn ? "player" : "hidden"}
            bind:this={player_canvas}
            width="24"
            height="38"
          />
          {#if drawing_player}
            <Loading withOverlay={false} />
          {/if}
        </section>
      </Column>
    </Row>
  </Grid>
  <!-- <section class="buttons--wrapper">
    <div class="buttons">
      <button on:click={() => uploadSprite("hair")}>Load Custom Hair</button>
      <button on:click={() => uploadSprite("hat")}>Load Custom Hat</button>
      <button on:click={() => uploadSprite("shirt")}>Load Custom Shirt</button>
      <button on:click={() => uploadSprite("pants")}>Load Custom Pants</button>
    </div>
    <div class="buttons">
      <button on:click={loadSave}>Open Save File</button>
      <button on:click={getContent}>Open Unpacked Content Path</button>
      <button on:click={drawCharacter}>Draw Character</button>
    </div>
  </section> -->
</main>

<style>
  @import "https://cdn.jsdelivr.net/npm/gridjs/dist/theme/mermaid.min.css";

  .hidden {
    opacity: 0 !important;
    border: none !important;
  }

  .player {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 312px;
    height: 494px;
    justify-self: center;
    image-rendering: pixelated;
    border: 2px solid #525252;
    border-radius: 3px;
    opacity: 1;
    transition: opacity 0.5s linear, border 0.5s linear;
  }

  .player--wrapper {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    align-items: center;
    width: 100%;
    min-height: 98vh;
  }

  .input--wrapper {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    height: 100%;
  }

  .inputs {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    gap: 1rem;
    height: 100%;
    padding-top: 3rem;
    padding-bottom: 3rem;
    max-height: 500px;
    max-width: 500px;
  }

  .button {
    position: relative;
    display: flex;
    align-self: flex-end;
    bottom: 0.5em;
  }

  .center {
    display: flex;
    flex-flow: column nowrap;
    align-items: center;
    align-content: space-around;
    width: 100%;
  }
</style>
