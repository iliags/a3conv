# Notes

## WDL

`BMAP name, <image name>, x, y, dx, dy`

The x and y are the upper left corner position, dx and dy are the width and height in pixels.

```none
REGION name {
 FLOOR_HGT 0;
 CEIL_HGT 20;
 FLOOR_TEX texture_name;
 CEIL_TEX texture_name;
 AMBIENT 0.6;
 IF_ENTER action;
 CLIP_DIST 300;
}
```
