use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;


pub fn load_default_album_art() -> Vec<u8> {
    // panicking here is actually OK -- the asset should be compiled with the binary.
    Assets::get("default_album_art.png")
        .unwrap()
        .data
        .into_owned()
}
