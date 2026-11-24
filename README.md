# YUNG's Better Ocean Monuments for Sunrise

Ocean monuments redesigned into larger, flooded temples with new chambers and guardians.

A port of [YUNG's Better Ocean Monuments](https://github.com/YUNG-GANG/YUNGs-Better-Ocean-Monuments) by YUNGNICKYOUNG and Acarii to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3). It carries the original's structures, templates, loot tables and strings, upgraded to 26.3, and a little Rust for its tridents. The 26.3 carry is by AdLf0208 ([AdLf0208/yungs_better_ocean_monuments](https://github.com/AdLf0208/yungs_better_ocean_monuments)); this fork moves it to Sunrise.

## What it adds

- A new ocean monument in deep oceans: domes, rings, a parthenon, maze and battle chambers, 59 templates assembled by YUNG's jigsaw rules.
- Its ten processors: the monument fills with water below sea level, its legs reach the sea floor, and its placeholders become slabs, sponges, seagrass and copper.
- Guardian spawns inside the monument, and its chest loot.
- The tridents lying in the monument never despawn; one a player picks up and throws despawns as usual.
- Vanilla ocean monuments no longer generate, as in the original (its "Disable Vanilla Ocean Monuments" option is on by default).

## Differences from the original

- The monument's tridents are kept by their owner alone: Sunrise can't yet tell whether a position is inside one of the monument's pieces, so one carried out of the monument stays too ([sunrise-org/sunrise#5305](https://github.com/sunrise-org/sunrise/issues/5305)).
- `/locate structure minecraft:ocean_monument` gives no hint to look for this monument instead, and underwater magma still generates inside it ([sunrise-org/sunrise#5266](https://github.com/sunrise-org/sunrise/issues/5266)).

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK) and Rust:

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/betteroceanmonuments.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-24-f8aa80e`.

## Licence and credits

YUNG's Better Ocean Monuments is by YUNGNICKYOUNG and Acarii (https://github.com/YUNG-GANG/YUNGs-Better-Ocean-Monuments). Its structures, templates, loot tables and strings are theirs and are used here under the **GNU LGPL v3.0 only**, the original's licence; `LICENSE` holds the text.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/yungs_better_ocean_monuments](https://github.com/AdLf0208/yungs_better_ocean_monuments)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
