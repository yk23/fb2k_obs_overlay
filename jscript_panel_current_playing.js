// JScript Panel 3 script - saves current track to JSON
var ENABLED = true;
var outputPath = fb.ProfilePath + "now_playing.json";

function on_playback_new_track(metadb) {
    writeNowPlaying(metadb, "playing");
}

function on_playback_stop(reason) {
    if (reason != 2) { // Not starting another track
        clearNowPlaying();
    }
}

function on_playback_pause(state) {
    var metadb = fb.GetNowPlaying();
    if (metadb) {
        writeNowPlaying(metadb, state ? "paused" : "playing");
    }
}


function writeFile(path, content) {
    try {
        var stream = new ActiveXObject("ADODB.Stream");
        stream.Type = 2; // Text
        stream.Charset = "UTF-8";
        stream.Open();
        stream.WriteText(content);
        stream.SaveToFile(path, 2); // Overwrite
        stream.Close();
    } catch(e) {
        fb.trace("Error writing file: " + e);
    }
}


function getMetaValue(fileInfo, fieldname) {
    try {
        return fileInfo.MetaValue(fileInfo.MetaFind(fieldname), 0) || "";
    } catch (error) {
        return ""
    }
}


function writeNowPlaying(metadb, playStatus) {
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
        data.play_status = playStatus;
        data.title = getMetaValue(fileInfo, "title");
        data.artist = getMetaValue(fileInfo, "album artist");
        data.album = getMetaValue(fileInfo, "album");
        data.track_number = getMetaValue(fileInfo, "tracknumber");
    }

    var json = JSON.stringify(data, null, 2);
    writeFile(outputPath, json);
}


function clearNowPlaying() {
    var emptyData = {
        play_status: "stopped",
        title: "",
        artist: "",
        album: "",
        track_number: "",
        duration: 0,
        file_path: ""
    };

    var json = JSON.stringify(emptyData, null, 2);
    writeFile(outputPath, json);
}

// Initialize on load
if (fb.IsPlaying) {
    writeNowPlaying(fb.GetNowPlaying(), "playing");
}