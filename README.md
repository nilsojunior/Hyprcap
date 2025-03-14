# Hyprcap

Screenshot utility for Hyprland.

You can take screenshots of windows, regions and monitors.

## Installation

Clone the repo.

```console
git clone https://github.com/nilsojunior/hyprcap.git
cd hyprcap
```

Build with cargo.

```console
cargo build --release
```

Add to path.

```console
sudo mv target/release/hyprcap /usr/bin
```

## Dependencies

- grim
- slurp
- wl-clipboard
- jq
- notification daemon

## Usage

See the help command.

```console
hyprcap -h
```

Screenshot a window.

```console
hyprcap -m window
```

You can move your cursor out of the screen for the screenshot.

```console
hyprcap -m window --move-cursor
```

## Hyprshade

If you use [Hyprshade](https://github.com/loqusion/hyprshade) you can
disable your current shader for the screenshot.

```console
   hyprcap -m window --disable-shader
```

## Save Directory

Hyprcap will look for the `HYPRCAP_DIR` enviroment variable to set a directory for
the screenshots.

If `HYPRCAP_DIR` is not set, it will look for `XDG_PICTURES_DIR`.

Iff `XDG_PICTURES_DIR` is not set, it will fallback to `HOME`.

## Credits

- Inspired by [Hyprshot](https://github.com/Gustash/Hyprshot)
- Check [Hyprshade](https://github.com/loqusion/hyprshade)
