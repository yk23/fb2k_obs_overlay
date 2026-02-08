let currentSongId = null;
let scrollTimerId = null;

let app_width = document.getElementById("app-container").clientWidth
let app_height = document.getElementById("app-container").clientHeight
// let album_art_width = Math.round(0.9 * app_width)
let album_art_height = Math.round(0.93 * app_height)
let album_art_width = album_art_height
console.log("Album art: width = " + album_art_width + " height = " + album_art_height)
document.getElementById("albumart").width = album_art_width
document.getElementById("albumart").height = album_art_height


async function updateNowPlaying() {
    const updateResponse = await fetch('/update', { method: 'POST' });
    if (!updateResponse.ok) {
        console.log(`POST failed: ${updateResponse.status}`);
        clearAll()
        return;
    }

    const metadataResponse = await fetch('/metadata', {
        method: 'GET',
        headers: {
            'Content-Type': 'applciation/json',
        }
    });

    if (!metadataResponse.ok) {
        console.log(`GET failed: ${metadataResponse.status}`);
        clearAll()
        return;
    }

    const metadata = await metadataResponse.json();

    if (metadata.status !== "none") {
        if (currentSongId == null || currentSongId !== metadata.song_id) {
            currentSongId = metadata.song_id
            updateDisplayFields(metadata)
            await updateAlbumArt();
        }
    } else {
        clearAll()
    }
}


function updateDisplayFields(metadata) {
    // Clear any existing scroll timer
    if (scrollTimerId) {
        clearTimeout(scrollTimerId);
        scrollTimerId = null;
    }

    // Animate text updates
    const updateWithAnimation = (id, value) => {
        const element = document.getElementById(id);
        element.classList.add('fade-in');
        element.textContent = value || '';
        setTimeout(() => element.classList.remove('fade-in'), 500);
    };

    updateWithAnimation('title', metadata.title);
    updateWithAnimation('artist', metadata.artist);
    updateWithAnimation('album', metadata.album);

    document.getElementById("song-details").classList.remove('hidden');
    document.getElementById("song-details").classList.add('fade-in');
    document.getElementById("song-nothing").classList.add('hidden');

    // Start scroll detection after a delay
    scrollTimerId = setTimeout(() => {
        const titleElement = document.getElementById('title');
        const wrapperElement = document.getElementById('title-wrapper');

        if (titleElement.scrollWidth > wrapperElement.clientWidth) {
            // Duplicate text with larger spacing for seamless loop
            titleElement.innerHTML = metadata.title + ' &nbsp;&nbsp;&nbsp;&nbsp; ' + metadata.title + ' &nbsp;&nbsp;&nbsp;&nbsp;';
            titleElement.classList.add('scrolling');
        } else {
            titleElement.classList.remove('scrolling');
        }
    }, 600);
}


async function updateAlbumArt() {
    const albumArtElement = document.getElementById('albumart');
    albumArtElement.src = `/album-art?song=${currentSongId}`;
}

function clearAll() {
    // Clear the scroll timer
    if (scrollTimerId) {
        clearTimeout(scrollTimerId);
        scrollTimerId = null;
    }

    // Remove scrolling class
    const titleElement = document.getElementById('title');
    titleElement.classList.remove('scrolling');

    // Hide everything, show default element.
    //
    document.getElementById("song-nothing").classList.remove('hidden');
    document.getElementById("song-nothing").classList.add('fade-in');
    document.getElementById("song-details").classList.add('hidden');
}
