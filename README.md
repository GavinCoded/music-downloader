# Music Downloader

A GTK4/libadwaita desktop app for searching and downloading music via yt-dlp.
<img width="950" height="700" alt="image" src="https://github.com/user-attachments/assets/bc86205c-f542-4dce-bde5-2b0c80b30b91" />


## Features

- Search for albums, artists, and tracks (**using deezer api**)
- Browse an artist's albums and view their tracklists
- Select and download full albums or individual tracks
- Embedded metadata
- Connect your Spotify account to browse your playlists and liked songs
- Download Spotify tracks by searching YouTube via yt-dlp

## Dependencies

- Rust (stable)
- GTK4 and libadwaita 1.4+
- `glib-compile-resources`
- `yt-dlp`
- `ffmpeg`


## Build

```sh
cargo build --release
```

## Note

This project has been worked on for months prior to being uploaded to GitHub. Some features may be broken or incomplete if you find any issues, feel free to open an issue or pull request.
