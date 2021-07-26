<script>
  // Custom utility functions
  import {
    cropImage,
    mirrorImage,
    fetchContent,
    generateSpriteDataUri,
    swapSkinColours,
    swapShoeColours,
    swapEyeColour,
    tintImage,
  } from "./utils";

  // Custom save file parser
  import { getSaveFile } from "./getSaveFile";

  // Libraries for dealing with colour
  import chroma from "chroma-js";
  import "vanilla-colorful";

  // Carbon Svelte components
  import {
    ComboBox,
    Grid,
    Row,
    Column,
    TextInput,
    Button,
    ProgressIndicator,
    Modal,
    ProgressStep,
    Loading,
    Slider,
  } from "carbon-components-svelte";

  // Carbon Icons
  import LicenseThirdParty32 from "carbon-icons-svelte/lib/LicenseThirdParty32";
  import Folder32 from "carbon-icons-svelte/lib/Folder32";
  import WatsonHealthHangingProtocol32 from "carbon-icons-svelte/lib/WatsonHealthHangingProtocol32";
  import TrashCan32 from "carbon-icons-svelte/lib/TrashCan32";
  import ColorPalette32 from "carbon-icons-svelte/lib/ColorPalette32";

  import ChevronDown32 from "carbon-icons-svelte/lib/ChevronDown32";
  import ChevronUp32 from "carbon-icons-svelte/lib/ChevronUp32";
  import Close32 from "carbon-icons-svelte/lib/Close32";

  // For handling dialog windows
  import { open as openDialog } from '@tauri-apps/api/dialog'

  // For handling titlebar interactivity
  import { appWindow } from '@tauri-apps/api/window'

  // The player object which will be filled with data from the savefile
  let player;

  // The direction the player is facing [0: Forwards, 1: Left, 2: Backwards, 3: Right]
  let player_direction = 0;

  // The object that will hold the sprites loaded from the unpacked content folder
  let sprites = {};

  // For holding custom sprites
  let custom_base = false;
  let custom_accessory = false;
  let custom_hair = false;
  let custom_hat = false;
  let custom_shirt = false;
  let custom_pants = false;

  // For holding the paths of custom sprites
  let custom_base_path,
    custom_accessory_path,
    custom_hair_path,
    custom_hat_path,
    custom_shirt_path,
    custom_pants_path;

  // These will hold the individual sprites used in the player drawing function
  let player_pants,
    player_hat,
    player_hair,
    player_body,
    player_accessory,
    player_arms,
    player_shirt;

  // Lists of colours for skin and shoes that will be filled by parsing the
  // skin colours and shoe colours sprites, respectively
  let player_skincolours = [];
  let player_shoecolours = [];

  // The JS canvas that will be used to draw the player
  let player_canvas;

  // For the progress bar under the load savefile and unpacked content buttons
  let progress_bar_index = 0;

  // The text paths to the savefile and unpacked content folders
  let save_file_path;
  let content_path;

  // Flags to indicate if the player is currently being drawn
  // Used to get the loading circle to pop up
  let drawing_player = false;
  let player_drawn = false;

  // Indices
  // "-1" means "none/default"; "0" means "the first in the list"
  let skin_colour_index = 0;
  let shoe_colour_index = 0;
  let shirt_index = -1;
  let pants_index = -1;
  let accessory_index = -1;
  let hat_index = -1;
  let hair_index = 0;

  // Customizable RGB colours
  let eye_colour;
  let hair_colour;
  let shirt_colour;
  let pants_colour;

  // Flags that open a colour picker modal for the corresponding custom colour
  let eye_colour_modal = false;
  let hair_colour_modal = false;
  let shirt_colour_modal = false;
  let pants_colour_modal = false;

  // Loads the savefile
  async function loadSave() {
    // A call to the custom getSaveFile function
    player = await getSaveFile(save_file_path);

    // If the getSaveFile function returned something
    if (player) {
      // Update the progress bar; next step is to load the unpacked content
      progress_bar_index = 1;
      // Update the savefile path variable
      save_file_path = player.saveFilePath;

      // Update Colours and Indices
      eye_colour = player.eyeColor;
      hair_colour = player.hairColor;

      shirt_colour = player.shirtColor;
      pants_colour = player.pantsColor;

      // Indices aren't colour strings; they're numbers
      // corresponding to rows in a sprite sheet
      skin_colour_index = player.skinColor;
      shoe_colour_index = player.shoes;

      // If the player shirt is not "-1" i.e. default
      if (player.shirtItem >= 0) {
        if (
          player.gender === "Female" &&
          player.shirtItem.indexInTileSheetFemale !== -1
        ) {
          shirt_index = player.shirtItem.indexInTileSheetFemale;
        } else {
          shirt_index = player.shirtItem.indexInTileSheet;
        }
      }

      // If the player pants are not "-1" i.e. default
      if (player.pantsItem >= 0) {
        pants_index = player.pantsItem.indexInTileSheet;
      }

      // Update the accessory index; might still be "-1"
      accessory_index = player.accessory;

      // Set the player's hat
      // If the index is out of range (i.e. for a custom hat),
      // the player's hat will be set to "-1" in updateHat()
      if (player.hat >= 0) {
        hat_index = player.hat.which;
      }

      // Set the player's hair index
      hair_index = player.hair;
    }
  }

  // Get the file path of the unpacked content folder
  async function getContent(event) {
    // If the existing path is not being used, open a dialog to get a new one
    if (event !== "use_existing_path") {
      content_path = await openDialog({ directory: true });
    }

    // Internal utility function to add a sprite to the global sprites object
    async function addSprite(spriteName) {
      const spriteToAdd = await fetchContent(
        `${content_path}/Characters/Farmer/${spriteName}.png`
      );

      sprites[spriteName] = generateSpriteDataUri(spriteToAdd);
    }

    // For each item in the list, add its sprite to the global sprites object
    // i.e. sprites.skinColours
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

      // Gets the player base sprite
      // Not sure how the bald logic works TBH
      // Maybe a custom base sprite should be an option in the future
      // (Although that would require a custom shoe drawing function)
      const farmerBase = await fetchContent(
        `${content_path}/Characters/Farmer/farmer_${
          player.gender === "Female" ? "girl_" : ""
        }base${player.hair === -1 ? "_bald" : ""}.png`
      );

      // Add the farmer base sprite fetched above, and increment
      // the progress bar (so now all buttons/sliders should be enabled)
      sprites.farmerBase = generateSpriteDataUri(farmerBase);
      progress_bar_index = 1;
    }
  }

  // What happens when you click the "Reset Appearance" button
  async function resetAppearance() {
    await loadSave();
    await getContent("use_existing_path");
  }

  // Basically just a big call-fest; see individual functions below
  // These should preferably be in separate files, but they're inpure functions
  // that mess with the global state, so...
  async function updateSprites() {
    await updateShoeColour();
    await updateSkinColour();
    await updateArms();
    await updateBody();
    await updateAccessory();
    await updateHair();
    await updateHats();
    await updateShirt();
    await updatePants();
  }

  // Update the player's shoe colour, using the current shoe colour
  // index and the shoe colours sprite
  async function updateShoeColour() {
    // Create a 4 x 1 OffscreenCanvas
    // Admittedly, using this API might be detrimental to
    // compatibility
    const shoe_canvas = new OffscreenCanvas(4, 1);
    const shoe_context = shoe_canvas.getContext("2d");

    // Init the shoe colours image to be updated below
    const shoe_colours_image = new Image();

    // Fetch the 4 x 1 sprite corresponding to the current
    // shoe colour index
    const shoe_colours_sprite = await cropImage(
      sprites.shoeColors,
      0,
      shoe_colour_index,
      4,
      1
    );

    // Set the src (source) attribute of the shoe colours image
    // to the sprite fetched above
    shoe_colours_image.src = shoe_colours_sprite;

    // Wait for the image's src to become available before
    // continuing (really useful method)
    await shoe_colours_image.decode();

    // Draw the shoe colours image onto the Offscreen Canvas
    shoe_context.drawImage(shoe_colours_image, 0, 0);

    function get_shoe_colours_data() {
      // Aw, yeah; making sync code async sorta!
      return new Promise((resolve) =>
        resolve(shoe_context.getImageData(0, 0, 4, 1).data)
      );
    }

    // Get the list of 16 numbers from reading the Offscreen Canvas
    // after drawing the shoe colours image to it
    // The numbers are in the format:
    // [R, G, B, A, R, G, B, A, R, G, B, A, R, G, B, A]
    const shoe_colours_data = await get_shoe_colours_data();

    // Reset the global shoe colours array
    player_shoecolours = [];

    // Push each of the above 16 values to a part of the shoe colours array
    // (put them into CSS colour strings while we're at it)
    player_shoecolours.push(
      `rgba(${shoe_colours_data[0]}, ${shoe_colours_data[1]}, ${
        shoe_colours_data[2]
      }, ${shoe_colours_data[3] / 255})`
    );
    player_shoecolours.push(
      `rgba(${shoe_colours_data[4]}, ${shoe_colours_data[5]}, ${
        shoe_colours_data[6]
      }, ${shoe_colours_data[7] / 255})`
    );
    player_shoecolours.push(
      `rgba(${shoe_colours_data[8]}, ${shoe_colours_data[9]}, ${
        shoe_colours_data[10]
      }, ${shoe_colours_data[11] / 255})`
    );
    player_shoecolours.push(
      `rgba(${shoe_colours_data[12]}, ${shoe_colours_data[13]}, ${
        shoe_colours_data[14]
      }, ${shoe_colours_data[15] / 255})`
    );
  }

  // Update the player's skin colour, using the current skin colour
  // index and the skin colours sprite
  async function updateSkinColour() {
    // Same Offscreen Canvas gimmick as with the shoe colours
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

    skin_colours_image.src = skin_colours_sprite;
    await skin_colours_image.decode();

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

    // Honestly, yeah; this is all pretty similar to the updateShoeColours() function
    // But there's a 3 x 1 sprite instead of a 4 x 1 sprite
    // It'd almost be worth genericizing this into one function. Almost...
  }

  // Get the players' arm sprite
  async function updateArms() {
    // Coordinates for cropping the arms sprite, based on the direction the player is facing
    // [0: Forwards, 1: Left, 2: Backwards, 3: Right]
    const arms_sprite_coords = [
      [96, 0],
      [96, 32],
      [96, 64],
      [96, 32],
    ];

    // The sprite to draw
    let arms_sprite = new Image();

    if (custom_base) {
      arms_sprite.src = custom_base;
    } else {
      arms_sprite.src = sprites.farmerBase;
    }

    await arms_sprite.decode();

    // Revert to the default base sprite if the custom one is not wide enough
    // (The assumption being that it doesn't have arms)
    if (arms_sprite.width <= 96) {
      arms_sprite = sprites.farmerBase;
    }

    // Crop out a particular bit of the player base sprite to get the front-facing
    // at-rest arms sprite
    let player_arms_sprite = await cropImage(
      sprites.farmerBase,
      arms_sprite_coords[player_direction][0],
      arms_sprite_coords[player_direction][1],
      16,
      32
    );

    // Obtain a left-facing sprite by mirroring a right-facing one
    if (player_direction === 1) {
      player_arms_sprite = await mirrorImage(player_arms_sprite);
    }

    // Run the fetched sprite through the custom swapSkinColours function
    // imported from utils.js
    const player_arms_updated_skin_colour = await swapSkinColours(
      player_arms_sprite,
      player_skincolours
    );

    // This trick is used in many places in this file, because it works really well
    player_arms = new Image();
    player_arms.src = player_arms_updated_skin_colour;
    await player_arms.decode();
  }

  // Get the player's body sprite (and update it to match the current skin colours index)
  async function updateBody() {
    let body_spritesheet = sprites.farmerBase;

    if (custom_base) {
      body_spritesheet = custom_base;
    }

    // Coordinates for cropping the body sprite, based on the direction the player is facing
    // [0: Forwards, 1: Left, 2: Backwards, 3: Right]
    const body_sprite_coords = [
      [0, 0],
      [0, 32],
      [0, 64],
      [0, 32],
    ];

    // Get the player body sprite
    let player_body_sprite = await cropImage(
      body_spritesheet,
      body_sprite_coords[player_direction][0],
      body_sprite_coords[player_direction][1],
      16,
      32
    );

    // Flip the body sprite if the player is facing left
    if (player_direction === 1) {
      player_body_sprite = await mirrorImage(player_body_sprite);
    }

    // Swap the skin colour of the body sprite if the skin colour index
    // isn't "0" - or the same as the vanilla sprite

    let player_body_updated_colours;

    if (skin_colour_index > 0) {
      player_body_updated_colours = await swapSkinColours(
        player_body_sprite,
        player_skincolours
      );
    } else {
      player_body_updated_colours = player_body_sprite;
    }

    player_body_updated_colours = await swapShoeColours(
      player_body_updated_colours,
      player_shoecolours
    );

    player_body_updated_colours = await swapEyeColour(
      player_body_updated_colours,
      eye_colour
    );

    // Either way, update the image with the (potentially) updated skin colour body sprite
    player_body = new Image();
    player_body.src = player_body_updated_colours;
    await player_body.decode();
  }

  // Update the player's accessory
  async function updateAccessory() {
    // Coordinates for cropping the accessory sprite, based on the direction the player is facing
    // [0: Forwards, 1: Left, 2: Backwards, 3: Right]

    // For use in cropping the accessory sprite, based on the direction the player is facing
    let accessory_offset = 0;

    // Left or right
    if (player_direction === 1 || player_direction === 3) {
      accessory_offset = 16;
    }

    // If the player is wearing a non-custom accessory
    if (accessory_index >= 0 && !custom_accessory) {
      // Some admittedly esoteric logic for finding the row and column of the given accessory
      const accessoryRow = Math.ceil((accessory_index + 1) / 8);
      const accessoryCol =
        accessory_index + 1 - Math.abs((accessoryRow - 1) * 8);

      // Fetch the accessory sprite
      const player_accessory_sprite = await cropImage(
        sprites.accessories,
        (accessoryCol - 1) * 16,
        (accessoryRow - 1) * 32 + accessory_offset,
        16,
        16
      );

      // Set the global accessory sprite
      player_accessory = new Image();

      // If the player is not facing backwards (i.e. no accessory)
      if (player_direction !== 2) {
        // Mirror if left
        if (player_direction === 1) {
          player_accessory.src = await mirrorImage(player_accessory_sprite);
        } else {
          player_accessory.src = player_accessory_sprite;
        }
      }

      await player_accessory.decode();
    } else if (custom_accessory) {
      // Fetch and set a custom accessory sprite
      const player_accessory_sprite = await cropImage(
        custom_accessory,
        0,
        0 + accessory_offset,
        16,
        16
      );

      player_accessory = new Image();

      // If the player is not facing backwards (i.e. no accessory)
      if (player_direction !== 2) {
        // Mirror if left
        if (player_direction === 1) {
          player_accessory.src = await mirrorImage(player_accessory_sprite);
        } else {
          player_accessory.src = player_accessory_sprite;
        }
      }

      await player_accessory.decode();
    } else {
      // Clear the accessory sprite
      player_accessory = new Image();
    }
  }

  // Update the player's hair sprite
  async function updateHair() {
    let hairRow = 1;
    let hairCol = 1;
    let hairX = 0;
    let hairY = 0;
    let hairSprite = sprites.hairstyles;

    // If the hair index is not -1 or custom
    // (It's possible for the hair index to be set too high initially)
    if (!custom_hair && hair_index >= 0) {
      // Remember, there are now two hairstyles sprite sheets
      if (hair_index < 56) {
        hairRow = Math.ceil((hair_index + 1) / 8);
        hairCol = hair_index + 1 - 8 * (hairRow - 1);

        hairX = (hairCol - 1) * 16;
        hairY = (hairRow - 1) * 96;
      } else {
        hairRow = Math.ceil((hair_index - 56 + 1) / 8);
        hairCol = hair_index - 56 + 1 - 8 * (hairRow - 1);

        hairX = (hairCol - 1) * 16;
        hairY = (hairRow - 1) * 96;
      }
    } else if (custom_hair) {
      hairSprite = custom_hair;
      hairX = 0;
      hairY = 0;
    } else {
      // If the player has no hair
      // Note that this doesn't change the base sprite to the bald variant, though it probably should
      hairSprite = false;
    }

    // If the player has a hair sprite
    if (hairSprite) {
      // Offset based on what direction the player is facing
      let hair_offset = 0;

      // Left or right
      if (player_direction === 1 || player_direction === 3) {
        hair_offset = 32;
      }

      // Backwards
      if (player_direction === 2) {
        hair_offset = 64;
      }

      // Crop it
      const player_hair_sprite = await cropImage(
        hairSprite,
        hairX,
        hairY + hair_offset,
        16,
        32
      );

      // Get the hair colour as an [R, G, B, A] array
      const hairTint = chroma(hair_colour).rgba();

      // Call the custom tintImage function in utils.js
      // with the R, G, and B from the hairTint
      const player_hair_tinted = await tintImage(
        player_hair_sprite,
        hairTint[0],
        hairTint[1],
        hairTint[2]
      );

      // Update the global hair sprite
      player_hair = new Image();

      // Flip if player is facing left
      if (player_direction === 1) {
        player_hair.src = await mirrorImage(player_hair_tinted);
      } else {
        player_hair.src = player_hair_tinted;
      }

      await player_hair.decode;
    } else {
      // Clear the global hair sprite
      player_hair = new Image();
    }
  }

  // Update the player's hat sprite
  async function updateHats() {
    let hatNum = hat_index;
    let hatSprite = sprites.hats;

    if (custom_hat) {
      // Reset the hat index to an arbitrary value in range
      hatNum = 1;
      // Change the hat sprite to the global custom hat sprite
      hatSprite = custom_hat;
    }

    // Check to ensure the hat index is in range
    if (hatNum < 94 && hatNum >= 0) {
      let hatRow = 1;
      let hatCol = 1;

      // Modify the row and column for a vanilla hat
      if (!custom_hat) {
        hatRow = Math.ceil((hatNum + 1) / 12);
        hatCol = hatNum - 12 * (hatRow - 1) + 1;
      }

      // Offset based on which direction the player is facing
      let hat_offset = 0;

      // Thought I'd... switch it up a bit...
      switch (player_direction) {
        // Left - yeah there actually is a left hat sprite; no mirroring required
        case 1:
          hat_offset = 40;
          break;
        // Backwards
        case 2:
          hat_offset = 60;
          break;
        // Right
        case 3:
          hat_offset = 20;
          break;
      }

      // Extract the hat sprite data
      const player_hat_sprite = await cropImage(
        hatSprite,
        (hatCol - 1) * 20,
        (hatRow - 1) * 80 + hat_offset,
        20,
        20
      );

      // Update the global hat sprite
      player_hat = new Image();
      player_hat.src = player_hat_sprite;
      await player_hat.decode();
    } else {
      // Clear the global hat sprite
      player_hat = new Image();
    }
  }

  // Update the player's shirt sprite
  // Admittedly, this one is a bit of a doozy
  async function updateShirt() {
    let player_shirt_sprite;

    // Internal function to fetch a shirt given an index
    async function fetchShirt(shirtNum) {
      const shirts = new Image();
      shirts.src = sprites.shirts;
      await shirts.decode();

      let x = 0;
      let y = 0;
      let shirtRow = 0;
      let shirtColumn = 0;

      let shirtOffset = 0;

      switch (player_direction) {
        // Left
        case 1:
          shirtOffset = 16;
          break;
        // Backwards
        case 2:
          shirtOffset = 24;
          break;
        // Right
        case 3:
          shirtOffset = 8;
          break;
      }

      // Note the Offscreen Canvas showing up again
      const canvas = new OffscreenCanvas(shirts.width, shirts.height);
      const context = canvas.getContext("2d");
      context.drawImage(shirts, 0, 0, shirts.width, shirts.height);

      // Row and column logic
      shirtRow = Math.ceil((shirtNum + 1) / 16);
      shirtColumn = shirtNum + 1 - Math.abs((shirtRow - 1) * 16);

      x = (shirtColumn - 1) * 8;
      y = (shirtRow - 1) * 32;

      // If the shirt is from the top part of the sprite sheet
      if (shirtNum < 128) {
        return await cropImage(sprites.shirts, x, y + shirtOffset, 8, 8);
      } else {
        // Otherwise, if the sprite isn't empty at a given index
        if (
          context
            .getImageData(x, y, 8, 32)
            .data.reduce((prev, val) => prev + val)
        ) {
          return await cropImage(sprites.shirts, x, y + shirtOffset, 8, 8);
          // If the sprite is empty
        } else {
          // If the sprite 128 pixels over isn't empty
          if (
            context
              .getImageData(x + 128, y, 8, 32)
              .data.reduce((prev, val) => prev + val)
          ) {
            return await cropImage(
              sprites.shirts,
              x + 128,
              y + shirtOffset,
              8,
              8
            );
          } else {
            // Return the default shirt sprite otherwise
            if (player.gender === "Female") {
              shirtRow = Math.ceil((130 + 1) / 16);
              shirtColumn = 130 + 1 - Math.abs((shirtRow - 1) * 16);
            } else {
              shirtRow = Math.ceil((41 + 1) / 16);
              shirtColumn = 41 + 1 - Math.abs((shirtRow - 1) * 16);
            }

            x = (shirtColumn - 1) * 8;
            y = (shirtRow - 1) * 32;
            return await cropImage(sprites.shirts, x, y + shirtOffset, 8, 8);
          }
        }
      }
    }

    // Actually fetching the shirt sprite
    if (custom_shirt) {
      player_shirt_sprite = await cropImage(custom_shirt, 0, 0, 8, 8);
    } else if (shirt_index >= 0 && shirt_index <= 299) {
      player_shirt_sprite = await fetchShirt(shirt_index);
    } else {
      if (player.gender === "Female") {
        // Default Female shirt
        player_shirt_sprite = await fetchShirt(130);
      } else {
        player_shirt_sprite = await fetchShirt(41);
      }
    }

    // Tint the shirt
    let shirtTint;

    if (shirt_colour) {
      // Convert the shirt_colour into an [R, G, B, A] array
      shirtTint = chroma(shirt_colour).rgba();
      // Tint the shirt sprite
      player_shirt_sprite = await tintImage(
        player_shirt_sprite,
        shirtTint[0],
        shirtTint[1],
        shirtTint[2]
      );
    }

    // Return the modified shirt sprite image
    player_shirt = new Image();
    player_shirt.src = player_shirt_sprite;
    await player_shirt.decode();
  }

  // Update the player's pants sprite
  async function updatePants() {
    let pants_sprite_sheet = sprites.pants;
    let player_pants_sprite;

    let pantsNum = 0;

    // Change the pantsNum if the index is in range
    if (pants_index >= 0 && pants_index <= 15) {
      pantsNum = pants_index;
    } else {
      pantsNum = 0;
    }

    // Use a custom pants sprite if provided
    if (custom_pants) {
      pants_sprite_sheet = custom_pants;
      pantsNum = 0;
    }

    let pantsRow = Math.ceil((pantsNum + 1) / 10);
    let pantsCol = pantsNum + 1 - 10 * (pantsRow - 1);

    // For cropping the pants sprite based on the player's direction
    let pantsOffset = 0;

    // Left or right
    if (player_direction === 1 || player_direction === 3) {
      pantsOffset = 32;
    }

    // Backwards
    if (player_direction === 2) {
      pantsOffset = 64;
    }

    // Crop the pants sprite
    player_pants_sprite = await cropImage(
      pants_sprite_sheet,
      (pantsCol - 1) * 192,
      (pantsRow - 1) * 688 + pantsOffset,
      16,
      32
    );

    // Tint the pants sprite
    if (pants_colour) {
      let pantsTint = chroma(pants_colour).rgba();

      player_pants_sprite = await tintImage(
        player_pants_sprite,
        pantsTint[0],
        pantsTint[1],
        pantsTint[2]
      );
    }

    // Mirror the sprite if the character is facing left
    if (player_direction === 1) {
      player_pants_sprite = await mirrorImage(player_pants_sprite);
    }

    // Return the modified pants sprite image
    player_pants = new Image();
    player_pants.src = player_pants_sprite;
    await player_pants.decode();
  }

  // Used for processing custom sprites
  async function uploadSprite(sprite_type) {
    // Open a dialog and fetch the content at the path provided
    const spritePath = await openDialog();
    const spriteData = await fetchContent(spritePath);

    // Filter through the potential options (from the Custom x: fields in the app)
    // Make use of the custom generateSpriteDataUri function from utils.js
    if (sprite_type === "base") {
      custom_base_path = spritePath;
      custom_base = generateSpriteDataUri(spriteData);
    } else if (sprite_type === "hair") {
      custom_hair_path = spritePath;
      custom_hair = generateSpriteDataUri(spriteData);
    } else if (sprite_type === "accessory") {
      custom_accessory_path = spritePath;
      custom_accessory = generateSpriteDataUri(spriteData);
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

  // Do the actual drawing of the character onto the canvas
  async function drawCharacter() {
    // Set some flags (for the loading spinner)
    drawing_player = true;
    player_drawn = false;

    // Call the updateSprites function and wait for it to finish executing
    await updateSprites();

    // Clear the player canvas
    const ctx = player_canvas.getContext("2d");
    ctx.clearRect(0, 0, player_canvas.width, player_canvas.height);

    // So the player sprite is centered
    const offsetX = 4;
    const offsetY = 1;

    // Drawing the body
    ctx.drawImage(player_body, 0 + offsetX, 0 + offsetY);

    // Draw the pants
    ctx.drawImage(player_pants, 0 + offsetX, 0 + offsetY);

    // Draw the shirt
    ctx.drawImage(player_shirt, 4 + offsetX, 16 + offsetY);

    // Draw the accessory
    ctx.drawImage(player_accessory, 0 + offsetX, 3 + offsetY);

    // Draw the hair
    ctx.drawImage(player_hair, 0 + offsetX, 0 + offsetY);

    // Draw the hat
    ctx.drawImage(player_hat, -2 + offsetX, -1 + offsetY);

    // Draw the arms
    ctx.drawImage(player_arms, 0 + offsetX, 0 + offsetY);

    // Set the flags so the loading spinner goes away
    drawing_player = false;
    player_drawn = true;
  }

  // Sets the Carbon UI theme to full dark
  $: document.documentElement.setAttribute("theme", "g100");
</script>

<div data-tauri-drag-region class="titlebar">
  <span>Stardew Valley Character Preview</span>
  <div class="titlebar-buttons">
  <div class="titlebar-button" on:click={() => appWindow.minimize()}>
    <ChevronDown32>
      <title>Minimize Window</title>
    </ChevronDown32>
  </div>
  <div class="titlebar-button" on:click={async (e) => { await appWindow.isMaximized() ? appWindow.unmaximmize() : appWindow.maximize() } }>
    <ChevronUp32>
      <title>Maximize Window</title>
    </ChevronUp32>
  </div>
  <div class="titlebar-button" on:click={() => appWindow.close()}>
    <Close32>
      <title>Close Window</title>
    </Close32>
  </div>
</div>
</div>

<main>
  <Grid>
    <!-- Wraps the top inputs and player canvas -->
    <section class="input--wrapper">
      <div class="inputs">
        <Row>
          <Column>
            <!-- A trick to get the text field and button(s) to sit on the same line nicely -->
            <Row>
              <!-- The savefile path -->
              <TextInput
                size="sm"
                disabled
                value={save_file_path ? save_file_path : ""}
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
              <!-- The unpacked content folder path -->
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
            <!-- The progress bar under the two top folder inputs -->
            <div class="progress">
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
            </div>

            <!-- Reset Appearance and Draw buttons -->
            <div class="character--buttons">
              <!-- Note how they're disabled unless the savefile and content path are loaded -->
              <button
                disabled={progress_bar_index < 1 || !content_path}
                on:click={resetAppearance}>Reset Appearance</button
              >
              <button
                disabled={progress_bar_index < 1 || !content_path}
                on:click={drawCharacter}>Draw Character</button
              >
            </div>
            <!-- Don't love how this looks; the removal buttons -->
            <!-- The default Carbon UI buttons were too big -->
            <section class="removal--buttons">
              <Column
                style="display: flex; flex-flow: column nowrap; justify-content: center; gap: 1rem;"
              >
                <!-- Note how "removing" something is just setting its index equal to "-1" -->
                <button
                  disabled={progress_bar_index < 1 ||
                    !content_path ||
                    custom_hat}
                  on:click={() => (hat_index = -1)}>Remove Hat</button
                >
                <button
                  disabled={progress_bar_index < 1 ||
                    !content_path ||
                    custom_shirt}
                  on:click={() => (shirt_index = -1)}>Remove Shirt</button
                >
              </Column><Column
                style="display: flex; flex-flow: column nowrap; justify-content: center; gap: 1rem;"
              >
                <button
                  disabled={progress_bar_index < 1 ||
                    !content_path ||
                    custom_pants}
                  on:click={() => (pants_index = -1)}>Remove Pants</button
                >
                <button
                  disabled={progress_bar_index < 1 || !content_path}
                  on:click={() => (accessory_index = -1)}>Remove Acc.</button
                >
              </Column>
              <Column
                style="display: flex; flex-flow: column nowrap; justify-content: center; gap: 1rem;"
              >
                <button
                  disabled={progress_bar_index < 1 ||
                    !content_path ||
                    custom_hair}
                  on:click={() => (hair_index = -1)}>Remove Hair</button
                >
              </Column>
            </section>
          </Column>
          <hr class="mini vertical" />
          <!-- The player canvas itself; right-click and "Save As" to save the preview image -->
          <Column>
            <section class="player--wrapper">
              <canvas
                class={player_drawn ? "player" : "hidden"}
                bind:this={player_canvas}
                width="24"
                height="38"
              />
              <!-- The loading circle -->
              {#if drawing_player}
                <Loading />
              {/if}
              <div class="direction--selector">
                <!-- The player direction selector box; yeah, there's probably a better way to do the placeholder -->
                <ComboBox
                  titleText="Direction"
                  direction="top"
                  size="sm"
                  disabled={!content_path}
                  placeholder={player_direction === 0
                    ? "Forwards"
                    : player_direction === 1
                    ? "Left"
                    : player_direction === 2
                    ? "Right"
                    : player_direction === 3
                    ? "Backwards"
                    : ""}
                  items={[
                    { id: "0", text: "Forwards" },
                    { id: "1", text: "Left" },
                    { id: "2", text: "Backwards" },
                    { id: "3", text: "Right" },
                  ]}
                  on:select={(event) => {
                    player_direction = event.detail.selectedIndex;
                  }}
                />
              </div>
            </section>
          </Column>
        </Row>
      </div>
    </section>

    <Row>
      <!-- The sliders -->
      <div class="sliders--wrapper">
        <section class="sliders">
          <!-- Note how the min, max, and disabled props work -->
          <Slider
            disabled={progress_bar_index < 1 || !content_path}
            labelText="Skin Colour"
            min={0}
            max={23}
            value={skin_colour_index}
            on:change={(event) => (skin_colour_index = event.detail)}
          />
          <!-- Also note how the on:change function works; pretty simple really -->
          <Slider
            disabled={progress_bar_index < 1 || !content_path}
            labelText="Shoe Colour"
            min={0}
            max={18}
            value={shoe_colour_index}
            on:change={(event) => (shoe_colour_index = event.detail)}
          />

          <Slider
            disabled={progress_bar_index < 1 || !content_path || custom_hair}
            labelText="Hair Index"
            min={-1}
            max={79}
            value={hair_index}
            on:change={(event) => (hair_index = event.detail)}
          />

          <Slider
            disabled={progress_bar_index < 1 || !content_path}
            labelText="Accessory Index"
            min={-1}
            max={18}
            value={accessory_index}
            on:change={(event) => (accessory_index = event.detail)}
          />

          <Slider
            disabled={progress_bar_index < 1 || !content_path || custom_hat}
            labelText="Hat Index"
            min={-1}
            max={93}
            value={hat_index}
            on:change={(event) => {
              hat_index = event.detail;
            }}
          />

          <Slider
            disabled={progress_bar_index < 1 || !content_path || custom_pants}
            labelText="Pants Index"
            min={-1}
            max={15}
            value={pants_index}
            on:change={(event) => (pants_index = event.detail)}
          />

          <Slider
            disabled={progress_bar_index < 1 || !content_path || custom_shirt}
            labelText="Shirt Index"
            min={-1}
            max={299}
            value={shirt_index}
            on:change={(event) => (shirt_index = event.detail)}
          />
        </section>
      </div>
    </Row>
    <Row>
      <Column>
        <!-- For loading in custom sprites -->
        <section class="custom--content">
          <Row>
            <TextInput
              size="sm"
              disabled
              value={custom_base_path ? custom_base_path : ""}
              labelText="Custom Base:"
            />
            <!-- Kinda jank way of making the buttons work side-by-side, but it looks presentable -->
            <div class="button">
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Browse"
                icon={WatsonHealthHangingProtocol32}
                on:click={() => uploadSprite("base")}
              />
              <Button
                size="small"
                kind="danger-tertiary"
                iconDescription="Remove Custom Base"
                disabled={!custom_base_path}
                icon={TrashCan32}
                on:click={() => {
                  custom_base = false;
                  custom_base_path = "";
                }}
              />
            </div>
          </Row>
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
                disabled={progress_bar_index < 1 || !content_path}
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
              value={custom_accessory_path ? custom_accessory_path : ""}
              labelText="Custom Accessory:"
            />
            <div class="button">
              <!-- One button adds a sprite, the other takes it away -->
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Browse"
                icon={WatsonHealthHangingProtocol32}
                on:click={() => uploadSprite("accessory")}
              />
              <Button
                size="small"
                kind="danger-tertiary"
                iconDescription="Remove Custom Accessory"
                disabled={!custom_accessory_path}
                icon={TrashCan32}
                on:click={() => {
                  custom_accessory = false;
                  custom_accessory_path = "";
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
                disabled={progress_bar_index < 1 || !content_path}
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
                disabled={progress_bar_index < 1 || !content_path}
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
                disabled={progress_bar_index < 1 || !content_path}
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
        </section>
      </Column>
      <Column>
        <!-- Each colour picker comes with its own text input, button, and modal -->
        <section class="colour--pickers">
          <Row>
            <!-- No button to reset the colour though; maybe there should be... -->
            <!-- Notice how the background colour is 15% opacity of the current colour -->
            <TextInput
              size="sm"
              disabled
              value={eye_colour ? eye_colour : ""}
              labelText="Eye Colour:"
              style={eye_colour
                ? `background-color: ${chroma(eye_colour).alpha(0.15)}`
                : ""}
            />
            <div class="button">
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Set Eye Colour"
                icon={ColorPalette32}
                on:click={() => (eye_colour_modal = true)}
              />
            </div>
          </Row>

          <!-- Here's the modal; note the open and on:close props -->
          <Modal
            passiveModal
            open={eye_colour_modal}
            on:close={() => {
              eye_colour_modal = false;
            }}
            modalHeading="Set Eye Colour"
          >
            <div class="modal--body">
              <!-- The colour picker itself; note how the colour gets updated -->
              <hex-color-picker
                on:color-changed={(event) => {
                  if (event.detail.value) {
                    const newColour = chroma(event.detail.value).rgba();
                    eye_colour = `rgba(${newColour[0]}, ${newColour[1]}, ${newColour[2]}, 1)`;
                  }
                }}
              />
            </div>
          </Modal>

          <Row>
            <TextInput
              size="sm"
              disabled
              value={hair_colour ? hair_colour : ""}
              labelText="Hair Colour:"
              style={hair_colour
                ? `background-color: ${chroma(hair_colour).alpha(0.15)}`
                : ""}
            />
            <div class="button">
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Set Hair Colour"
                icon={ColorPalette32}
                on:click={() => (hair_colour_modal = true)}
              />
            </div>
          </Row>

          <Modal
            passiveModal
            open={hair_colour_modal}
            on:close={() => {
              hair_colour_modal = false;
            }}
            modalHeading="Set Hair Colour"
          >
            <div class="modal--body">
              <hex-color-picker
                on:color-changed={(event) => {
                  if (event.detail.value) {
                    const newColour = chroma(event.detail.value).rgba();
                    hair_colour = `rgba(${newColour[0]}, ${newColour[1]}, ${newColour[2]}, 1)`;
                  }
                }}
              />
            </div>
          </Modal>

          <Row>
            <TextInput
              size="sm"
              disabled
              value={shirt_colour ? shirt_colour : ""}
              labelText="Shirt Colour:"
              style={shirt_colour
                ? `background-color: ${chroma(shirt_colour).alpha(0.15)}`
                : ""}
            />
            <div class="button">
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Set Shirt Colour"
                icon={ColorPalette32}
                on:click={() => (shirt_colour_modal = true)}
              />
              <Button
              size="small"
              kind="danger-tertiary"
              iconDescription="Remove Shirt Colour"
              disabled={progress_bar_index < 1 || !content_path || !shirt_colour}
              icon={TrashCan32}
              on:click={() => {
                shirt_colour = "";
              }}
            />
            </div>

          </Row>

          <Modal
            passiveModal
            open={shirt_colour_modal}
            on:close={() => {
              shirt_colour_modal = false;
            }}
            modalHeading="Set Shirt Colour"
          >
            <div class="modal--body">
              <hex-color-picker
                on:color-changed={(event) => {
                  if (event.detail.value) {
                    const newColour = chroma(event.detail.value).rgba();
                    shirt_colour = `rgba(${newColour[0]}, ${newColour[1]}, ${newColour[2]}, 1)`;
                  }
                }}
              />
            </div>
          </Modal>

          <Row>
            <TextInput
              size="sm"
              disabled
              value={pants_colour ? pants_colour : ""}
              labelText="Pants Colour:"
              style={pants_colour
                ? `background-color: ${chroma(pants_colour).alpha(0.15)}`
                : ""}
            />
            <div class="button">
              <Button
                size="small"
                disabled={progress_bar_index < 1 || !content_path}
                iconDescription="Set Pants Colour"
                icon={ColorPalette32}
                on:click={() => (pants_colour_modal = true)}
              />
              <Button
              size="small"
              kind="danger-tertiary"
              iconDescription="Remove Pants Colour"
              disabled={progress_bar_index < 1 || !content_path || !pants_colour}
              icon={TrashCan32}
              on:click={() => {
                pants_colour = "";
              }}
            />
            </div>
          </Row>

          <Modal
            passiveModal
            open={pants_colour_modal}
            on:close={() => {
              pants_colour_modal = false;
            }}
            modalHeading="Set Pants Colour"
          >
            <div class="modal--body">
              <hex-color-picker
                on:color-changed={(event) => {
                  if (event.detail.value) {
                    const newColour = chroma(event.detail.value).rgba();
                    pants_colour = `rgba(${newColour[0]}, ${newColour[1]}, ${newColour[2]}, 1)`;
                  }
                }}
              />
            </div>
          </Modal>
        </section>
      </Column>
    </Row>
  </Grid>
</main>

<style>
  /* The CSS for this app */

  main {
    width: 100vw;
  }

  /* For the canvas when it's not supposed to be visible */
  .hidden {
    opacity: 0 !important;
    border: 2px solid #525252 !important;
    width: 312px;
    height: 494px;
  }

  /* The next two are for the player canvas and its wrapper */
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
    height: 100%;
    width: 100%;
    margin-bottom: 2rem;
  }

  /* Custom button implementation; not too impressive but it does the job */
  .button {
    position: relative;
    display: flex;
    align-self: flex-end;
    bottom: 0.5em;
  }

  button {
    background-color: #262626;
    color: white;
    padding: 0.5rem;
    font-weight: bold;
  }

  button:disabled {
    font-weight: normal;
    background-color: #161616;
    color: #8d8d8d;
  }

  button:disabled:hover {
    cursor: not-allowed;
  }

  button:hover {
    cursor: pointer;
  }

  button:focus {
    border: 1px solid #0f62fe;
  }

  button:active {
    background-color: #363636;
  }

  /* Dividers; note the vertical class (it isn't shown on smaller screens) */
  hr {
    border: 1px solid hsl(219, 99%, 33%);
    margin-bottom: 2rem;
  }

  hr.mini {
    border: 1px solid hsl(219, 99%, 13%);
    margin: 2rem;
  }

  @media only screen and (max-width: 729px) {
    hr.vertical {
      display: none;
    }
  }

  /* The "Draw Character" and "Reset Appearance" buttons */
  .character--buttons {
    display: flex;
    flex-flow: row nowrap;
    align-items: center;
    justify-content: center;
    margin-top: 2rem;
    margin-bottom: 2rem;
    gap: 1rem;
  }

  /* The progress bar */
  .progress {
    margin-top: 2rem;
  }

  /* The "Remove X" buttons */
  .removal--buttons {
    display: flex;
    flex-flow: column nowrap;
    align-items: center;
    justify-content: center;
  }

  /* The selector box for the direction the player is facing */
  .direction--selector {
    margin: 0;
    padding: 0;
  }

  /* The wrapper for the "Custom X:" inputs */
  .custom--content {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    height: 100%;
    padding-left: 2rem;
    padding-right: 2rem;
  }

  /* The wrapper for the colour inputs */
  .colour--pickers {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    height: 100%;
    padding-left: 2rem;
    padding-right: 2rem;
  }

  /* Making the inside of the modals look a bit better */
  .modal--body {
    position: relative;
    display: flex;
    flex-flow: column nowrap;
    align-items: center;
    overflow: hidden;
    left: 15%;
  }

  /* A bit of a margin for the inputs */
  .input--wrapper {
    margin-top: 2rem;
  }

  /* Sliders container and container wrapper */
  .sliders {
    display: flex;
    flex-flow: row wrap;
    justify-content: center;
    align-items: center;
    margin-top: 1rem;
    margin-bottom: 1rem;
    gap: 2rem;
    max-width: 800px;
  }

  .sliders--wrapper {
    display: flex;
    flex-flow: column nowrap;
    justify-content: center;
    align-items: center;
    margin-left: 0.25rem;
    margin-right: 1rem;
    width: 100%;
  }

  /* A couple global styles */
  :global(.bx--slider-container) {
    width: 100%;
  }

  /* Fixes the direction selector's arrow somehow */
  :global(.bx--list-box__menu-icon) {
    transform: translateY(-25%);
  }

  :global(html) {
    overflow-x: hidden;
  }
</style>
