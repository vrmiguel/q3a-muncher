# q3a-muncher ![bfg](https://icons.iconarchive.com/icons/th3-prophetman/game/48/Quake-III-Arena-icon.png)

`q3a-muncher` parses through a Quake 3 logfile and builds reports for every game.

## Examples

```bash
cat sample.log
  1:08 Kill: 3 2 6: Isgalamido killed Mocinha by MOD_ROCKET
  1:10 Item: 3 item_armor_combat
  1:11 Item: 4 weapon_shotgun
  1:18 Item: 4 weapon_rocketlauncher
  1:18 Item: 4 ammo_rockets
  1:26 Kill: 1022 4 22: <world> killed Zeh by MOD_TRIGGER_HURT
  1:29 Item: 3 weapon_railgun
  1:32 Item: 3 weapon_railgun
  1:32 Kill: 1022 4 22: <world> killed Zeh by MOD_TRIGGER_HURT
  1:35 Item: 2 item_armor_shard
  1:35 Item: 3 weapon_railgun
  1:38 Item: 3 weapon_railgun
  1:41 Kill: 1022 2 19: <world> killed Dono da Bola by MOD_FALLING
  1:41 Item: 3 weapon_railgun
  1:45 Kill: 1022 2 19: Mocinha killed Dono da Bola by MOD_MACHINEGUN
  1:47 Item: 2 item_armor_shard
  1:47 ShutdownGame:
```

`q3a-muncher sample.log` results in

```json
"game0": {
    "total_kills": 5,
    "players": ["Mocinha", "Isgalamido", "Zeh", "Dono da Bola"],
    "kills": {
        "Mocinha": 1,
        "Isgalamido": 1,
        "Zeh": -2,
        "Dono da Bola": -1
    },
    "kills_by_means": {
        "MOD_SHOTGUN": 0,
        "MOD_GAUNTLET": 0,
        "MOD_MACHINEGUN": 1,
        "MOD_GRENADE": 0,
        "MOD_GRENADE_SPLASH": 0,
        "MOD_ROCKET": 1,
        "MOD_ROCKET_SPLASH": 0,
        "MOD_PLASMA": 0,
        "MOD_PLASMA_SPLASH": 0,
        "MOD_RAILGUN": 0,
        "MOD_LIGHTNING": 0,
        "MOD_BFG": 0,
        "MOD_BFG_SPLASH": 0,
        "MOD_WATER": 0,
        "MOD_SLIME": 0,
        "MOD_LAVA": 0,
        "MOD_CRUSH": 0,
        "MOD_TELEFRAG": 0,
        "MOD_FALLING": 1,
        "MOD_SUICIDE": 0,
        "MOD_TARGET_LASER": 0,
        "MOD_TRIGGER_HURT": 2,
        "MOD_NAIL": 0,
        "MOD_CHAINGUN": 0,
        "MOD_PROXIMITY_MINE": 0,
        "MOD_KAMIKAZE": 0,
        "MOD_JUICED": 0,
        "MOD_GRAPPLE": 0,
        "MOD_UNKNOWN": 0
    }
}
```
