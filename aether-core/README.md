# Aether Core - Minecraft Bot for 2b2t

This bot is created to server the main purpose of loading the pearl on 2b2t.

## TODOs

- [x] Better way to send chat messages with delay
- [x] Integration with Discord
- [x] Split Code into multiple plugins
- [x] Support Multiple Bots
- [ ] Add Bots via discord with certain roles
- [ ] Add More Roles like `Storage Manager`, `AFK at Farms`, etc.
- [ ] Update `config.json` via commands

## Features

This sections aims to explain the features/settings in [`config.example.json`](./config.example.json)

| Feature             | Description                                                                                                                                       | Role based |
|---------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|------------|
| `log_bridge`        | Sends logs received to the discord channel                                                                                                        | No         |
| `chat_bridge`       | Sends all the chat received to discord channel                                                                                                    | Yes        |
| `queue_bridge`      | Sends queue position to discord channel. _NOTE: This feature is still incomplete and designed only for 2b2t._                                     | Yes        |
| `join_coordination` | Attempts to join the bots in a certain way so that they can be online for maximum time. Each bot using this should have `coordinate` set to true. | Yes        |