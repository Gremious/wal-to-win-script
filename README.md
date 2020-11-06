# wal-to-win-script
Takes a [pywal](https://github.com/dylanaraps/pywal) json and converts it into a [Windows Terminal](https://docs.microsoft.com/en-us/windows/terminal/customize-settings/color-schemes) compatible format. You can run this with [rust-script](https://rust-script.org/#installation).

Example:

```json
{
    "wallpaper": "Some\\path\\here",
    "alpha": "100",
    "special": {
        "background": "#252439",
        "foreground": "#c8c1e3",
        "cursor": "#c8c1e3"
    },
    "colors": {
        "color0": "#252439",
        "color1": "#8B77C5",
        "color2": "#6D93DE",
        "color3": "#FC9493",
        "color4": "#F19FA6",
        "color5": "#F594AF",
        "color6": "#DEB4BA",
        "color7": "#c8c1e3",
        "color8": "#8c879e",
        "color9": "#8B77C5",
        "color10": "#6D93DE",
        "color11": "#FC9493",
        "color12": "#F19FA6",
        "color13": "#F594AF",
        "color14": "#DEB4BA",
        "color15": "#c8c1e3"
    }
}
```

to

```json
{
  "name": "illusion_theme",
  "cursorColor": "#c8c1e3",
  "selectionBackground": "#c8c1e3",
  "background": "#252439",
  "foreground": "#c8c1e3",
  "black": "#252439",
  "blue": "#8B77C5",
  "cyan": "#6D93DE",
  "green": "#FC9493",
  "purple": "#F19FA6",
  "red": "#F594AF",
  "white": "#DEB4BA",
  "yellow": "#c8c1e3",
  "brightBlack": "#8c879e",
  "brightBlue": "#8B77C5",
  "brightCyan": "#6D93DE",
  "brightGreen": "#FC9493",
  "brightPurple": "#F19FA6",
  "brightRed": "#F594AF",
  "brightWhite": "#DEB4BA",
  "brightYellow": "#c8c1e3"
}
```
