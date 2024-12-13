# a3conv

A set of extraction and conversion tools for Acknex 3 games. The goal is to convert the assets into a format which is usable in modern game engines (i.e. Unreal, Unity, Godot, Bevy, etc.).

## Progress

This is extremely WIP, it won't work under most circumstances for the time being. What does work is the archive extraction portion which is an alternative to QuickBMS.

- [x] Extract WRS archives
- [ ] Convert *.pcx images into other formats
- [ ] Convert maps (*.wmp) into other formats

The map files contain vertices, regions, walls, and objects. They rely on wdl scripts for logic, special effects, geometry tweaking, and other tasks which increase the complexity.

## Related Projects

- [A3Tools](https://github.com/firoball/A3Tools)
- [WMPio](https://github.com/firoball/WMPio)
- [uWed](https://github.com/firoball/uWED)

## Quick BMS Script

Use this script with QuickBMS to extract a WRS archive.

```none
comtype lzss
endian big
get asize asize
do
getdstring name 13
get zsize long
get size long
savepos offset
clog name offset zsize size
math offset += zsize
goto offset
while offset < asize
```
