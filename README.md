# a3conv

A set of extraction and conversion tools for Acknex 3 games. The goal is to convert the assets into a format which is usable in modern game engines (i.e. Unreal, Unity, Godot, Bevy, etc.).

## Usage

Use ```a3conv_cli -g [path to game] -c``` to have the program detect all archive file types and do the extraction/conversion to an output folder in the same directory. Removing the ```-c``` will skip converting files and use ```-h``` flag to see options.

## Progress

This is extremely early in development, things will probably break. Currently it can extract .wrs archives and convert .pcx images to either PNG (default) or JPEG.

## Customized Crates

Contains a source version of [image](https://crates.io/crates/image) because the PCX image format is temporarily disabled in 0.25.5.

## Related Projects

- [A3Tools](https://github.com/firoball/A3Tools)
- [WMPio](https://github.com/firoball/WMPio)
- [uWed](https://github.com/firoball/uWED)
