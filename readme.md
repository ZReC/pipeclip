# Pipeclip

Currently, this software has the sole purpose of serializing whatever image|text you have in the clipboard through the standard output.\
The image format of choice is called [Portable Arbitrary Map](https://en.wikipedia.org/wiki/Netpbm#PAM_graphics_format).

## Build

```sh
cargo build --release
```

The executable will be located at `target/release/pipeclip`

## Usage

This program was meant to be used in conjunction with FFmpeg or similar:

```sh
pipeclip | ffmpeg -f pam_pipe -i - .webp
```

Though, you could use it to dump the contents into a file:

```sh
pipeclip > .pam
ffplay -f pam_pipe -i .pam
```

## Acknowledgment

This app is cross-platform thanks to [arboard](https://github.com/1Password/arboard).

## License

[See here](license.md)