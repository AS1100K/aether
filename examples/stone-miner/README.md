# Stone Miner

This Minecraft bot mines the stone above it more efficiently. The bot uses combination of `Forward`, `Backward`, 
`Left`, and `Right` movements.

This `README.md` files was created to help in creating & understanding `config.json`.

[Demo Video](https://www.youtube.com/watch?v=oG_ljRwTT8k)

```json
{
  "server": "localhost:12345",
  "mode": "Offline",
  "username": "_aether",
  "checkpoints": [
    [-1067, 4, -422],
    [-1067, 4, -413],
    [-1066, 4, -413],
    [-1066, 4, -422]
  ],
  "directions": [
    "Right",
    "Forward",
    "Left",
    "Backward"
  ],
  "initial_y_rot": 0.0
}
```

The above code block is the sample `config.json` and some fields are self-explanatory.

`initial_y_rot` -> The `y_rot` for the bot after it is standing on the first checkpoint i.e. the direction of the 
second checkpoint.

`directions` -> This contains list movements. When writing the movement you need to first figure out how will the bot 
be moving. For example: The bot is on the first checkpoint and needs to move to the next checkpoint i.e. 
`directions[1]`. The direction here will tell the bot which way to move in order to get to that checkpoint from the 
previous checkpoint.
