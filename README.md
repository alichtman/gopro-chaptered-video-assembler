# GoPro Chaptered Video Combiner

When you record long videos on GoPro, the videos get split into 4GB chunks.

https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US

etc etc etc


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
