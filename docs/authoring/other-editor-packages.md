# Web wallpapers and other packages in the editor

The editor also opens and creates packages that are not scenes. In *Scenes*, the *Web page* starting
point creates a folder with `index.html`, `app.js` (a clock that takes its color from its property
and stops when the page is hidden) and a valid `web` manifest; *Open a project…* and the copies in
the library also open web pages, images, videos and presets. The package page shows the list of
files with their role (the entry page, the script that runs first, the preview), their size and
the total; *Add files…* copies them into the folder and each file offers to be used as the entry
page or as the thumbnail. Problems, undo, save, *Set as wallpaper* and *Publish…* work as they do
in scenes.

```sh
reiwand-ctl scene new clock --from web --title Clock   # index.html, app.js and the manifest
reiwand-ctl scene open clock                           # the files with their role and size
reiwand-ctl scene act clock manifest '{"add_file":{"file":"background.png","to":"img"}}'   # copies a file into the folder
```
