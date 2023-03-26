# GoPro Chaptered Video Assembler

When you record long-form videos on a GoPro, the videos get split into 4GB chunks. In the [file naming conventions spec](https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US), GoPro refers to these as chaptered video files.

The general format is:

```
GXYYZZZZ.mp4, where:
 X is the encoding type (X for HEVC, H for AVC .... yes, I know)
  YY is the chapter number
    ZZZZ is the video number
```

Here is an example GoPro video (about ~10min long) that was split into three chapters.

```
 test_data
├──  GX010119.MP4 -- 3.7GB
├──  GX020119.MP4 -- 3.7GB
└──  GX030119.MP4 -- 3.7GB
```

After being combined, using the `concat demuxer` method detailed [here](https://stackoverflow.com/a/11175851), the output file is: `GoPro_119.mp4`, sized at 10GB.

## Which models are supported?

- HERO11 Black / Black Mini
- HERO10 Black
- HERO9 Black
- HERO8 Black
- HERO7 (White, Silver, Black)
- HERO6 Black

## Installation

```bash
$ cargo install gopro-chaptered-video-assembler
```

## Usage

```bash
$ gopro-chaptered-video-assembler -h
╔═══════════════════════════════════════╗
║ gopro-chaptered-video-assembler 0.1.0 ║
╚═══════════════════════════════════════╝
Assembles all chaptered GoPro video files in a directory into 'complete' files.
Written by: Aaron Lichtman
Source: https://github.com/alichtman/gopro-chaptered-video-assembler

Usage: gopro-chaptered-video-assembler [OPTIONS]

Options:
  -i, --input-dir <DIRECTORY>   Directory to parse video files from
  -o, --output-dir <DIRECTORY>  Directory to output video files to
  -h, --help                    Print help
  -V, --version                 Print version

```
