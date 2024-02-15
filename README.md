# GoPro Chaptered Video Assembler

**TL;DR: Work with your GoPro files as you recorded them, not as the camera stored them.**

## Quickstart

```
# Install from cargo
$ cargo install gopro-chaptered-video-assembler

# Run the tool
$ gopro-chaptered-video-assembler --input PATH --output PATH
```

## Why does this exist?

I've been using my GoPro to record videos for years. I shoot a lot of long-form climbing and mountain videos. Any video longer than a few minutes gets split into multiple files, which are unpleasant to deal with when it's time to edit. It's a tedious and time-consuming process to reassemble the videos, and I'd rather spend my time doing literally anything else.

So I wrote a Rust tool to reassemble them for me. 

It is easy to use, lossless (thanks to [`mp4-merge`](https://github.com/gyroflow/mp4-merge)), built with user safety in mind (will never delete your data without asking), and has integration tests.

## How does it work?

When you record long-form videos on a GoPro, the videos get split into `~4GB` chunks. In the [file naming conventions spec](https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US), GoPro refers to these as chaptered video files.

The naming scheme is:

```bash
GXYYZZZZ.mp4, where:
 X       is the encoding type (X for HEVC, H for AVC .... yes... I know)
  YY     is the chapter number
    ZZZZ is the video number
```

Here is an example directory structure for a single GoPro video (about ~10min long) that was split into three chapters.

```bash
gopro-chaptered-video-example/
├── GX010119.MP4 [Video 0119, chapter 1]
├── GX020119.MP4 [Video 0119, chapter 2]
└── GX030119.MP4 [Video 0119, chapter 3]
```

Here's an example directory structure with multiple chaptered videos.

```bash
gopro-multiple-chaptered-videos-example/
├── GH017455.MP4 [Video 7455, chapter 1]
├── GH017456.MP4 [Video 7456, chapter 1]
├── GH017457.MP4 [Video 7457, chapter 1]
├── GH027455.MP4 [Video 7455, chapter 2]
├── GH027456.MP4 [Video 7456, chapter 2]
└── GH037455.MP4 [Video 7455, chapter 2]
```

Instead of thinking about _which-video-goes-where_ myself, I outsource it to this tool.

## So what does the tool give you?

One single output directory that contains your GoPro footage, with easy filenames. All output files will have the form: `GoPro_{video_number}.MP4`.

### For Multichapter Videos...

It finds and combines multi-chapter videos using [`mp4-merge`](https://github.com/gyroflow/mp4-merge). If a multi-chapter merge operation is done, a set of commands will be printed at the end to clean up the original source directory. These commands are destructive, and therefore need to be run manually.

### For Single Chapter Videos...

It renames, or copies (if you use `--no-single-chapter-rename`), single chapter videos.

## Installation

This package is available on [`crates.io`](https://crates.io/crates/gopro-chaptered-video-assembler).

```bash
$ cargo install gopro-chaptered-video-assembler
```

### Building and Installing Source

```
$ git clone https://github.com/alichtman/gopro-chaptered-video-assembler
$ cd gopro-chaptered-video-assembler

# Compile
$ cargo build

# Run dev build
$ cargo run -- [ARGUMENTS]

# Install to machine
$ cargo install --path .
```

Debugging in VSCode works well.

## Example Usage

```bash
$ gopro-chaptered-video-assembler --input "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/" --output output
╔═══════════════════════════════════════╗
║ gopro-chaptered-video-assembler 0.5.1 ║
╚═══════════════════════════════════════╝
01:49:33 [INFO] Found 39 files in directory: /media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure
01:49:33 [INFO] Found 5 video(s) with 11 total chapters to combine
01:49:33 [INFO] {
    7515: [
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH017515.MP4",
            video_number: 7515,
            chapter: 1,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH027515.MP4",
            video_number: 7515,
            chapter: 2,
        },
    ],
    7517: [
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH017517.MP4",
            video_number: 7517,
            chapter: 1,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH027517.MP4",
            video_number: 7517,
            chapter: 2,
        },
    ],
    7511: [
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH017511.MP4",
            video_number: 7511,
            chapter: 1,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH027511.MP4",
            video_number: 7511,
            chapter: 2,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH037511.MP4",
            video_number: 7511,
            chapter: 3,
        },
    ],
    7477: [
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH017477.MP4",
            video_number: 7477,
            chapter: 1,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH027477.MP4",
            video_number: 7477,
            chapter: 2,
        },
    ],
    7485: [
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH017485.MP4",
            video_number: 7485,
            chapter: 1,
        },
        GoProChapteredVideoFile {
            abs_path: "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/GH027485.MP4",
            video_number: 7485,
            chapter: 2,
        },
    ],
}
01:49:33 [INFO] And 27 single chapter video(s) to rename
Proceed? (y/n)
...
```

The `output` directory will be created (in the current directory if a relative path is given, or at an absolute path otherwise), and all output files will be found there.

Additionally, paths with apostrophes (and other special characters) are supported:

```bash
$ gopro-chaptered-video-assembler -i "tests/working_test_data/Test\'s Apostrophe"  -o tests/output
...
```

## Which GoPro models are supported?

- HERO11 Black / Black Mini
- HERO10 Black
- HERO9 Black
- HERO8 Black
- HERO7 White / Silver / Black
- HERO6 Black
