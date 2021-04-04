<script>
  import { createEventDispatcher } from "svelte";
  import { getSaveFile } from "./getSaveFile";

  import {
    cropImage,
    tintImage,
    swapSkinColours,
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
    Slider,
  } from "carbon-components-svelte";

  import LicenseThirdParty32 from "carbon-icons-svelte/lib/LicenseThirdParty32";

  import Folder32 from "carbon-icons-svelte/lib/Folder32";

  import WatsonHealthHangingProtocol32 from "carbon-icons-svelte/lib/WatsonHealthHangingProtocol32";
  import TrashCan32 from "carbon-icons-svelte/lib/TrashCan32";

  let player;

  let sprites = {};

  let custom_hat = false;
  let custom_hair = false;
  let custom_shirt = false;
  let custom_pants = false;

  let custom_hat_path, custom_hair_path, custom_shirt_path, custom_pants_path;

  let player_pants,
    player_hat,
    player_hair,
    player_body,
    player_accessory,
    player_arms,
    player_shirt;

  let player_skincolours = [];
  let player_shoecolours = [];

  let player_canvas;

  let progress_bar_index = 0;

  let content_path = "";

  let drawing_player = false;
  let player_drawn = false;

  // Indices
  let skin_colour_index;
  let shoe_colour_index;
  let shirt_index;
  let pants_index;
  let accessory_index;
  let hat_index;
  let hair_index;

  // Customizable RGBA colours
  let eye_colour;
  let hair_colour;
  let shirt_colour;
  let pants_colour;

  const dispatcher = createEventDispatcher();

  async function loadSave() {
    player = await getSaveFile();
    if (player) {
      console.dir(player);
      progress_bar_index = 1;

      // Update Colours and Indices
      eye_colour = player.eyeColor;
      hair_colour = player.hairColor;
      shirt_colour = player.shirtColor;
      pants_colour = player.pantsColor;

      skin_colour_index = player.skinColor;
      shoe_colour_index = player.shoes;

      if (player.shirtItem) {
        if (
          player.gender === "Female" &&
          player.shirtItem.indexInTileSheetFemale !== -1
        ) {
          shirt_index = player.shirtItem.indexInTileSheetFemale;
        } else {
          shirt_index = player.shirtItem.indexInTileSheet;
        }
      }

      if (player.pantsItem) {
        pants_index = player.pantsItem.indexInTileSheet;
      }

      accessory_index = player.accessory;

      if (player.hat) {
        hat_index = player.hat.which;
      }

      hair_index = player.hair;
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
        "hairstyles2",
      ].forEach((sprite) => {
        addSprite(sprite);
      });

      // Not sure how the bald logic works TBH
      const farmerBase = await fetchContent(
        `${content_path}/Characters/Farmer/farmer_${
          player.gender === "Female" ? "girl_" : ""
        }base${player.hair === -1 ? "_bald" : ""}.png`
      );

      sprites.farmerBase = generateSpriteDataUri(farmerBase);
      progress_bar_index = 1;
    }
  }

  async function updateSprites() {
    await updateSkinColour();
    await updateShoeColour();
    await updateArms();
    await updateBody();
    await updateHats();
    await updateAccessories();
    await updateHair();
    await updateShirt();
    await updatePants();
  }

  async function updateSkinColour() {
    const skin_canvas = new OffscreenCanvas(3, 1);
    const skin_context = skin_canvas.getContext("2d");
    const skin_colours_image = new Image();

    const skin_colours_sprite = await cropImage(
      sprites.skinColors,
      0,
      skin_colour_index,
      3,
      1
    );

    let skin_colours_array = [];

    skin_colours_image.onload = () => {
      skin_context.drawImage(skin_colours_image, 0, 0);

      const skin_colours_data = skin_context
        .getImageData(0, 0, 3, 1)
        .data.map((item) => {
          skin_colours_array.push(item);
        });

      player_skincolours = [];

      player_skincolours.push(
        `rgba(${skin_colours_array[0]}, ${skin_colours_array[1]}, ${
          skin_colours_array[2]
        }, ${skin_colours_array[3] / 255})`
      );
      player_skincolours.push(
        `rgba(${skin_colours_array[4]}, ${skin_colours_array[5]}, ${
          skin_colours_array[6]
        }, ${skin_colours_array[7] / 255})`
      );
      player_skincolours.push(
        `rgba(${skin_colours_array[8]}, ${skin_colours_array[9]}, ${
          skin_colours_array[10]
        }, ${skin_colours_array[11] / 255})`
      );
    };

    skin_colours_image.src = skin_colours_sprite;
  }

  async function updateShoeColour() {
    const shoe_canvas = new OffscreenCanvas(4, 1);
    const shoe_context = shoe_canvas.getContext("2d");

    const shoe_colours_image = new Image();

    const shoe_colours_sprite = await cropImage(
      sprites.shoeColors,
      0,
      shoe_colour_index,
      4,
      1
    );

    let shoe_colours_array = [];

    shoe_colours_image.onload = () => {
      shoe_context.drawImage(shoe_colours_image, 0, 0);

      const shoe_colours_data = shoe_context
        .getImageData(0, 0, 4, 1)
        .data.map((item) => {
          shoe_colours_array.push(item);
        });

      player_shoecolours.push(
        `rgba(${shoe_colours_array[0]}, ${shoe_colours_array[1]}, ${
          shoe_colours_array[2]
        }, ${shoe_colours_array[3] / 255})`
      );
      player_shoecolours.push(
        `rgba(${shoe_colours_array[4]}, ${shoe_colours_array[5]}, ${
          shoe_colours_array[6]
        }, ${shoe_colours_array[7] / 255})`
      );
      player_shoecolours.push(
        `rgba(${shoe_colours_array[8]}, ${shoe_colours_array[9]}, ${
          shoe_colours_array[10]
        }, ${shoe_colours_array[11] / 255})`
      );
      player_shoecolours.push(
        `rgba(${shoe_colours_array[12]}, ${shoe_colours_array[13]}, ${
          shoe_colours_array[14]
        }, ${shoe_colours_array[15] / 255})`
      );
    };

    shoe_colours_image.src = shoe_colours_sprite;
  }

  async function updateHair() {
    let hairRow = 1;
    let hairCol = 1;
    let hairX = 0;
    let hairY = 0;
    let hairSprite = sprites.hairstyles;

    if (!custom_hair) {
      if (hair_index < 56) {
        hairCol = Math.ceil((hair_index + 1) / 4);
        hairRow = hair_index + 1 - Math.abs((hairCol - 1) * 4);

        hairX = (hairCol - 1) * 16;
        hairY = hairRow * 96;
      } else {
        hairCol = Math.ceil((hair_index - 56 + 1) / 4);
        hairRow = hair_index - 56 + 1 - Math.abs((hairCol - 1) * 4);
        hairSprite = sprites.hairstyles2;

        hairX = (hairCol - 1) * 16;
        hairY = (hairRow - 1) * 96;
      }
    } else {
      hairSprite = custom_hair;
      hairX = 0;
      hairY = 0;
    }

    const player_hair_sprite = await cropImage(
      hairSprite,
      hairX,
      hairY,
      16,
      32
    );

    const hairTint = chroma(hair_colour).rgba();

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
    let hatNum = hat_index;
    let hatSprite = sprites.hats;

    if (custom_hat) {
      hatNum = 1;
      hatSprite = custom_hat;
    }

    if (hatNum) {
      let hatRow = 1;
      let hatCol = 1;

      if (!custom_hat) {
        hatRow = Math.ceil((hatNum + 1) / 12);
        hatCol = hatNum - 12 * (hatRow - 1) + 1;
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

    const player_arms_updated_skin_colour = await swapSkinColours(
      player_arms_sprite,
      player_skincolours
    );

    player_arms = new Image();
    player_arms.src = player_arms_updated_skin_colour;
  }

  async function updateBody() {
    const player_body_sprite = await cropImage(
      sprites.farmerBase,
      0,
      0,
      16,
      32
    );

    const player_body_updated_skin_colour = await swapSkinColours(
      player_body_sprite,
      player_skincolours
    );

    player_body = new Image();
    player_body.src = player_body_updated_skin_colour;
  }

  async function updateAccessories() {
    if (accessory_index > 0) {
      const accessoryRow = Math.ceil(accessory_index / 8);
      const accessoryCol = accessory_index - Math.abs((accessoryRow - 1) * 8);

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

    let pantsNum = 0;

    if (pants_index) {
      pantsNum = pants_index;
    } else {
      pantsNum = 0;
    }

    if (custom_pants) {
      pants_sprite_sheet = custom_pants;
      pantsNum = 0;
    }

    let pantsRow = Math.ceil((pantsNum + 1) / 10);
    let pantsCol = pantsNum + 1 - 10 * (pantsRow - 1);

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

    async function fetchShirt(shirtNum) {
      const shirts = new Image();
      shirts.src = sprites.shirts;

      let x = 0;
      let y = 0;
      let row = 0;
      let column = 0;

      const canvas = new OffscreenCanvas(shirts.width, shirts.height);
      const context = canvas.getContext("2d");
      context.drawImage(shirts, 0, 0, shirts.width, shirts.height);

      row = Math.ceil((shirtNum + 1) / 16);
      column = shirtNum + 1 - Math.abs((row - 1) * 16);

      x = (column - 1) * 8;
      y = (row - 1) * 32;

      if (shirtNum < 128) {
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
    } else if (shirt_index) {
      player_shirt_sprite = await fetchShirt(shirt_index);
    } else {
      if (player.gender === "Female") {
        // Default Female shirt
        player_shirt_sprite = await fetchShirt(130);
      } else {
        player_shirt_sprite = await fetchShirt(41);
      }
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
    ctx.clearRect(0, 0, player_canvas.width, player_canvas.height);

    const offsetX = 4;
    const offsetY = 1;

    ctx.drawImage(player_body, 0 + offsetX, 0 + offsetY);

    ctx.fillStyle = eye_colour;
    ctx.fillRect(6 + offsetX, 12 + offsetY, 1, 2);
    ctx.fillRect(9 + offsetX, 12 + offsetY, 1, 2);

    const bootsOffsetX = 7;
    const bootsOffsetY = 28;

    ctx.fillStyle = player_shoecolours[0];
    ctx.fillRect(1 + bootsOffsetX, 0 + bootsOffsetY, 1, 3);
    ctx.fillRect(0 + bootsOffsetX, 3 + bootsOffsetY, 1, 2);
    ctx.fillRect(1 + bootsOffsetX, 4 + bootsOffsetY, 2, 1);
    ctx.fillRect(3 + bootsOffsetX, 3 + bootsOffsetY, 1, 1);
    ctx.fillRect(4 + bootsOffsetX, 0 + bootsOffsetY, 2, 3);
    ctx.fillRect(8 + bootsOffsetX, 0 + bootsOffsetY, 1, 3);
    ctx.fillRect(6 + bootsOffsetX, 3 + bootsOffsetY, 1, 1);
    ctx.fillRect(7 + bootsOffsetX, 4 + bootsOffsetY, 3, 1);
    ctx.fillRect(9 + bootsOffsetX, 3 + bootsOffsetY, 1, 1);

    ctx.fillStyle = player_shoecolours[1];
    ctx.fillRect(3 + bootsOffsetX, 1 + bootsOffsetY, 1, 2);
    ctx.fillRect(6 + bootsOffsetX, 1 + bootsOffsetY, 1, 2);

    ctx.fillStyle = player_shoecolours[2];
    ctx.fillRect(2 + bootsOffsetX, 1 + bootsOffsetY, 1, 3);
    ctx.fillRect(7 + bootsOffsetX, 1 + bootsOffsetY, 1, 3);

    ctx.fillStyle = player_shoecolours[3];
    ctx.fillRect(1 + bootsOffsetX, 3 + bootsOffsetY, 1, 1);
    ctx.fillRect(8 + bootsOffsetX, 3 + bootsOffsetY, 1, 1);

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
  <Grid fullWidth>
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
                <Button
                  size="small"
                  kind="danger-tertiary"
                  iconDescription="Remove Custom Hair"
                  disabled={!custom_hair_path}
                  icon={TrashCan32}
                  on:click={() => {
                    custom_hair = false;
                    custom_hair_path = "";
                  }}
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
                <Button
                  size="small"
                  kind="danger-tertiary"
                  iconDescription="Remove Custom Hat"
                  disabled={!custom_hat_path}
                  icon={TrashCan32}
                  on:click={() => {
                    custom_hat = false;
                    custom_hat_path = "";
                  }}
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
                <Button
                  size="small"
                  kind="danger-tertiary"
                  iconDescription="Remove Custom Shirt"
                  disabled={!custom_shirt_path}
                  icon={TrashCan32}
                  on:click={() => {
                    custom_shirt = false;
                    custom_shirt_path = "";
                  }}
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
                <Button
                  size="small"
                  kind="danger-tertiary"
                  iconDescription="Remove Custom Pants"
                  disabled={!custom_pants_path}
                  icon={TrashCan32}
                  on:click={() => {
                    custom_pants = false;
                    custom_pants_path = "";
                  }}
                />
              </div>
            </Row>
          </div>
        </section>
      </Column>
    </Row>
  </Grid>

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

  <Grid>
    <Row>
      <Column>
  <Slider
    labelText="Skin Colour"
    min={0}
    max={22}
    value={skin_colour_index}
    on:change={(event) => (skin_colour_index = event.detail)}
  />

  <Slider
    labelText="Shoe Colour"
    min={0}
    max={18}
    value={shoe_colour_index}
    on:change={(event) => (shoe_colour_index = event.detail)}
  />

  <Slider
    labelText="Shirt Index"
    min={0}
    max={299}
    value={shirt_index}
    disabled={custom_shirt}
    on:change={(event) => (shirt_index = event.detail)}
  />

  <Slider
    labelText="Pants Index"
    min={0}
    max={15}
    value={pants_index}
    disabled={custom_pants}
    on:change={(event) => (pants_index = event.detail)}
  />

  <Slider
    labelText="Accessory Index"
    min={0}
    max={18}
    value={accessory_index}
    on:change={(event) => (accessory_index = event.detail)}
  />

  <Slider
    labelText="Hat Index"
    min={0}
    max={93}
    value={hat_index}
    disabled={custom_hat}
    on:change={(event) => {
      hat_index = event.detail;
      console.log(hat_index);
    }}
  />

  <section class="removal--buttons">
    <button disabled={custom_hat} on:click={() => (hat_index = -1)}
      >Remove Hat</button
    >
    <button on:click={() => (accessory_index = -1)}>Remove Accessory</button>
    <button disabled={custom_shirt} on:click={() => (shirt_index = -1)}
      >Remove Shirt</button
    >
    <button disabled={custom_pants} on:click={() => (pants_index = -1)}
      >Remove Pants</button
    >
  </section>
</Column>
</Row>
  </Grid>
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

  .inputs {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    gap: 1rem;
    padding-top: 3rem;
    padding-bottom: 3rem;
  }

  .button {
    position: relative;
    display: flex;
    align-self: flex-end;
    bottom: 0.5em;
  }
</style>
