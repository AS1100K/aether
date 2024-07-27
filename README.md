<center>
    <img src="https://img.shields.io/github/license/AS1100K/aether"/>
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/AS1100K/aether/ci.yml?event=push&label=CI">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/AS1100K/aether/pr.yml?event=pull_request&label=PR">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/AS1100K/aether/release.yml?event=push&label=RELEASE">
</center>

# Aether
A Collection of Minecraft Bots written in rust mainly developed for `2b2t.org`.

> [!NOTE]
> This repository currently supports minecraft version `1.20.6` and uses self-maintained version of [azalea better-1.20.6](https://github.com/as1100k-forks/azalea.git)
> which has more features and fixes.

## Plugins
1. [Anti AFK](./plugins/anti-afk) Anti AFK Plugin with advanced configuration which attempts to not get the bot afk kicked.
2. [Auto Mine](./plugins/auto-mine) Left Click implementation, when enabled this block will mine anything it is looking at
if reachable. Also available in [#156](https://github.com/azalea-rs/azalea/pull/156) PR in azalea.
4. [Task Manager](./plugins/task-manager) A small task manager that executes task one by one. Currently, it only supports,
a small number of tasks.
5. [Utility](./plugins/utility) A small collection of plugins that are essential for bot survival. It includes Auto Eat, and
Kill Aura (WIP).

## Examples
1. [Anti AFK](./examples/anti-afk) This bot attempts to join the server, and simply tries to not get AFK kicked and also
    not die of hunger.
2. [Stone Miner](./examples/stone-miner) This bot is specifically designed for mining stones in a stone generator at
craftmc.pl for my friend.

## TODOs / Future Goals
_For Project related TODOs head to the project's `README.md`_

- [ ] Discord bot to receive commands to add a bot
- [ ] Map Art Bot which can create map arts
- [ ] Storage Manager Bot which can manage storage
- [ ] amethyst Crystal Farmer bot
- [ ] Auto Trading Bot which can trade with villagers and also, make armors
- [ ] Advanced Pathfinding [Example](https://github.com/adepierre/Botcraft/blob/master/Visuals/pathfinding_climb.gif)
