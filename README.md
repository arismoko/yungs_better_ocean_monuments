# YUNG's Better Ocean Monuments for Sunrise

Ocean monuments redesigned into larger, flooded temples with new chambers and guardians.

A port of [YUNG's Better Ocean Monuments](https://github.com/YUNG-GANG/YUNGs-Better-Ocean-Monuments) by YUNGNICKYOUNG and Acarii to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3). It is a content mod: the original's structures, templates, loot tables and strings, upgraded to 26.3, with no code. The 26.3 carry is by AdLf0208 ([AdLf0208/yungs_better_ocean_monuments](https://github.com/AdLf0208/yungs_better_ocean_monuments)); this fork moves it to Sunrise.

## What it adds

- A new ocean monument in deep oceans: domes, rings, a parthenon, maze and battle chambers, 59 templates assembled by YUNG's jigsaw rules.
- Its ten processors: the monument fills with water below sea level, its legs reach the sea floor, and its placeholders become slabs, sponges, seagrass and copper.
- Guardian spawns inside the monument, and its chest loot.
- Vanilla ocean monuments no longer generate, as in the original (its "Disable Vanilla Ocean Monuments" option is on by default).

## Differences from the original

- Tridents thrown in monument water do not persist; that is Java code.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK):

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/betteroceanmonuments.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-22-7cfdf81`.

## Licence and credits

YUNG's Better Ocean Monuments is by YUNGNICKYOUNG and Acarii (https://github.com/YUNG-GANG/YUNGs-Better-Ocean-Monuments). Its structures, templates, loot tables and strings are theirs and are used here under the **GNU LGPL v3.0 only**, the original's licence; `LICENSE` holds the text.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/yungs_better_ocean_monuments](https://github.com/AdLf0208/yungs_better_ocean_monuments)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
