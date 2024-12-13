# a3conv

A set of extraction and conversion tools for Acknex 3 games. The goal is to convert the assets into a format which is usable in modern game engines (i.e. Unreal, Unity, Godot, Bevy, etc.).

## Progress

- [x] Extract WRS archives
- [ ] Convert *.pcx images into other formats
- [ ] Convert maps (*.wmp) into other formats

The map files contain vertices, regions, walls, and objects. They rely on wdl scripts for logic, special effects, geometry tweaking, and other tasks which increases the complexity.
