![maintenance-status](https://img.shields.io/badge/maintenance-deprecated-red.svg)
![License](https://img.shields.io/github/license/AS1100K/aether)
![CI Status](https://img.shields.io/github/actions/workflow/status/AS1100K/aether/ci.yml?event=push&label=CI)
![PR Status](https://img.shields.io/github/actions/workflow/status/AS1100K/aether/pr.yml?event=pull_request&label=PR)
![Release Status](https://img.shields.io/github/actions/workflow/status/AS1100K/aether/release.yml?event=push&label=RELEASE)

# Aether
A Collection of Minecraft Bots written in rust mainly developed for `2b2t.org`.

> [!WARNING]
> This repository is archived i.e. it will no longer be maintained. This is done as the project has moved to
> [SpaceBots](https://github.com/spacebots-org) for providing high quality bots.
>
> These bot should remain functional but may contains bugs and can break due to breaking changes in azalea.

## Multi Azalea
This project is the wrapper around azalea to support multiple versions of azalea. This was made so that our plugins
support multiple versions of azalea. For more information see [multi-azalea](./multi-azalea/README.md)

## Plugins
1. [Anti AFK](./plugins/anti-afk) Plugin with advanced configuration which attempts to not get the bot afk kicked.
2. [Task Manager](./plugins/task-manager) A small task manager that executes task one by one. Currently, it only supports,
   a small number of tasks.
3. [Utility](./plugins/utility) A small collection of plugins that are essential for bot survival. It includes Auto Eat, and
   Kill Aura (WIP).

## Examples
1. [Anti AFK](./examples/anti-afk) This bot attempts to join the server, and simply tries to not get AFK kicked and also
    not die of hunger.
2. [Stone Miner](./examples/stone-miner) This bot is specifically designed for mining stones in a stone generator at `craftmc.pl`
   for my friend.

### Deprecated Plugins ![maintenance-status](https://img.shields.io/badge/maintenance-deprecated-red.svg)
* [Auto-Mine](https://github.com/AS1100K/aether/tree/deprecated/plugins/auto-mine) plugin is the implementation of holding left-click in minecraft. Also, available at [PR #168](https://github.com/azalea-rs/azalea/pull/168) of azalea.

## TODOs / Future Goals
_For Project related TODOs head to the project's `README.md`_

- [ ] Discord bot to receive commands to add a bot
- [ ] Map Art Bot which can create map arts
- [ ] Storage Manager Bot which can manage storage
- [ ] amethyst Crystal Farmer bot
- [ ] Auto Trading Bot which can trade with villagers and also, make armors
- [ ] Advanced Pathfinding [Example](https://github.com/adepierre/Botcraft/blob/master/Visuals/pathfinding_climb.gif)
