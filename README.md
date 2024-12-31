# AoS2 Save Editor

Save editor for [Acceleration of SUGURI 2](https://store.steampowered.com/app/390710/Acceleration_of_SUGURI_2/).
Gives no competitive advantage.

## What is it for?

If any of that describes you:

- Lost the save file, but don't wanna grind to unlock characters.
- Tired of rolling Sham if you main Random Character.
- Want to remove Iru from the game.
- Simply want all the things here and now.

Then this tool is for you. ![:ohh_yeah:](https://steamcommunity-a.akamaihd.net/economy/emoticon/:ohh_yeah:)

With it you can do the following:

- Unlock locked characters.
- Lock characters unlocked by default - no more Sham or Iru.
- Unlock all music and arena backgrounds.
- (Planned for future) Unlock all customization options for a player's online profile.

However:

- You CANNOT unlock DLC music.
  - It's checked on the steam client, not in the savefile.
    That's intended and there's no way around it.
- You CANNOT unlock DLC costumes.
  - Same reason.

![Only three of us left](./docs/readme/disabled_characters.jpg)

## Why this project exists

Here are all the reasons:

- AoS2 is the best game ever made.
- I wanted to cook a non-useless Rust project.
- Always wanted to make some TUI app cos `nvim` is cute and inspiring.

## How to contribute

I haven't been able to figure out all the fields in the savefile.
[`easydiff`](./easydiff/) crate in this workspace can help with that.

Simply leave an issue or open the PR if you have something to add.
