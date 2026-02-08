# fb2k_obs_overlay

A simple "Now Playing" browser-based overlay for OBS.

Integrates with a local media library! The only requirement is that songs are locally distributed into folders 
(e.g. albums) that contain a cover image (either 'cover', 'folder' or 'front', with .jpg or .png extensions.)

Example: 
```
<arbitrary parent structure>/
|-- my_album/
    |-- track1.mp4
    |-- track2.mp4
    |-- track3.mp4
    |-- cover.jpg
```

# How to use

1. Install a portable version of [foobar2000](https://www.foobar2000.org/), along with the "JScript Panel 3" extension.
2. Add the JScript panel to the foobar2000 GUI, and copy-paste the provided `jscript_panel_current_playing.js`.
3. Run the provided executable, noting the foobar2000's output `now_playing.json` file that appears when you start song playback. e.g. on Windows:
```
> NowPlayingWebOverlay.exe --json-path <FB2k_INSTALLATION_DIR>\\profile\\now_playing.json

Listening on 127.0.0.1:8082
Overlay art (app_width, app_height) = (900, 200)
```
4. Open OBS. In the "Sources" panel, navigate to "Add Source > Browser". 
In the URL field, type in the complete URL (with port) from step 3.
Type in the Width and Height dimensions as well. NOTE: you may need to adjust it slightly by adding +25 to both,
because the raw values may result in slight clipping.

Lastly, take note of the options, which can be viewed with `--help`:
```
Usage: NowPlayingWebOverlay.exe [OPTIONS] --json-path <FILE>

Options:
  -j, --json-path <FILE>  The path to the "now playing" JSON metadata log file. Use the provided jscript_panel_current_playing.js to produce it. Note that the path may depend in whether or not you're using a portable installation of foobar2000
  -i, --ip-bind <IP>      The local IP address to bind to [default: 127.0.0.1]
  -p, --port <PORT>       The local port to bind to [default: 8082]
      --width <WIDTH>     The browser overlay display width [default: 900]
      --height <HEIGHT>   The browser overlay display height [default: 200]
  -h, --help              Print help
  -V, --version           Print version

```