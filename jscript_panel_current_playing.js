// JScript Panel 3 script - saves current track to JSON
// For use in Foobar2000 --- https://hydrogenaudio.org/index.php/topic,110516.msg1067716.html#msg1067716
var ENABLED = true;
var outputPath = fb.ProfilePath + "now_playing.json";

function on_playback_new_track(metadb) {
    writeNowPlaying(metadb);
}

function on_playback_stop(reason) {
    if (reason != 2) { // Not starting another track
        clearNowPlaying();
    }
}

function writeNowPlaying(metadb) {
    if (!ENABLED)
        return;
    if (!metadb)
        return;

    var data = {
        title: metadb.RawPath,
        artist: "",
        album: "",
        track_number: "",
        duration: metadb.Length,
        file_path: metadb.Path
    };

    // Get metadata
    var fileInfo = metadb.GetFileInfo();
    if (fileInfo) {
        data.status = "playing";
        data.title = fileInfo.MetaValue(fileInfo.MetaFind("title"), 0) || "";
        data.artist = fileInfo.MetaValue(fileInfo.MetaFind("artist"), 0) || "";
        data.album = fileInfo.MetaValue(fileInfo.MetaFind("album"), 0) || "";
        data.track_number = fileInfo.MetaValue(fileInfo.MetaFind("tracknumber"), 0) || "";
    }

    var json = JSON.stringify(data, null, 2);

    try {
        var fso = new ActiveXObject("Scripting.FileSystemObject");
        var file = fso.CreateTextFile(outputPath, true);
        file.Write(json);
        file.Close();
    } catch(e) {
        fb.trace("Error writing JSON: " + e);
    }
}

function clearNowPlaying() {
    var emptyData = {
        status: "stopped",
        title: "",
        artist: "",
        album: "",
        track_number: "",
        duration: 0,
        file_path: ""
    };

    var json = JSON.stringify(emptyData, null, 2);

    try {
        var fso = new ActiveXObject("Scripting.FileSystemObject");
        var file = fso.CreateTextFile(outputPath, true);
        file.Write(json);
        file.Close();
    } catch(e) {
        fb.trace("Error clearing JSON: " + e);
    }
}

// Initialize on load
if (fb.IsPlaying) {
    writeNowPlaying(fb.GetNowPlaying());
}