# AoS2 Save Editor

Save editor for [Acceleration of SUGURI 2](https://store.steampowered.com/app/390710/Acceleration_of_SUGURI_2/).
Gives no competitive advantage.

Supported game version: `ver 1.9.0`

## Download

Look for [the latest release here](https://github.com/DOCtorActoAntohich/aos2-save-editor/releases)

## What is it for?

If any of that describes you:

- Lost the save file, but don't wanna grind to unlock characters.
- Tired of rolling Sham if you main Random Character.
- Want to remove Iru from the game.
- Simply want all the things here and now.

Then this tool is for you. ![:ohh_yeah:](https://steamcommunity-a.akamaihd.net/economy/emoticon/:ohh_yeah:)

### Features

With this app, you can do the following:

- Unlock locked characters.
- Lock characters unlocked by default - no more Sham or Iru.
- Unlock all music and arena backgrounds.
- Use all customization options for your online profile.
  - Skip the training and become Sham Specialist.
  - Use title and character combinations that are
    not available from the game UI,
    including hidden 100O% OJ goobers.

However:

- You CANNOT unlock DLC music.
  - It's checked on the steam client, not in the savefile.
    That's intended and there's no way around it.
- You CANNOT unlock DLC costumes.
  - Same reason.

![UI demo](./docs/readme/ui-demo.jpg)

![Secret Background](./docs/readme/super_secret_background.jpg)

![Only three of us left](./docs/readme/disabled_characters.jpg)

![Free Specialist Title (With Nanako)](./docs/readme/sham_specialist.jpg)

## Why this project exists

Here are all the reasons:

- AoS2 is the best game ever made.
- I wanted to cook a non-useless Rust project.
- Always wanted to make some TUI app cos `nvim` is cute and inspiring.

## Issues

A few problems to maybe solve in the future.

- I haven't tested if it works on Linux.
- I haven't been able to figure out all the fields in the savefile.
  [`easydiff`](./crates/easydiff/) crate in this workspace can help with that.

## Rust version

Use `1.85` or newer.

## License

Licensed under [MIT license](./LICENSE)

## Credits

Hono is genius.

He drew cool girls and I screenshotted one for the app Icon.
