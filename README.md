# GoPro Chaptered Video Assembler

## Why does this exist?

When you record long-form videos on a GoPro, the videos get split into `~4GB` chunks. In the [file naming conventions spec](https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US), GoPro refers to these as chaptered video files.

The general format is:

```bash
GXYYZZZZ.mp4, where:
 X is the encoding type (X for HEVC, H for AVC .... yes... I know)
  YY is the chapter number
    ZZZZ is the video number
```

Here is an example directory structure for a single GoPro video (about ~10min long) that was split into three chapters.

```bash
gopro-chaptered-video-example/
├── GX010119.MP4 [Video 1, chapter 1]
├── GX020119.MP4 [Video 1, chapter 2]
└── GX030119.MP4 [Video 1, chapter 3]
```

Here's an example directory structure with multiple chaptered videos.

```bash
gopro-multiple-chaptered-videos-example/
├── GH017455.MP4 [Video 1, chapter 1]
├── GH017456.MP4 [Video 2, chapter 1]
├── GH017457.MP4 [Video 3, chapter 1]
├── GH027455.MP4 [Video 1, chapter 2]
├── GH027456.MP4 [Video 2, chapter 2]
└── GH037455.MP4 [Video 3, chapter 2]
```

The file names aren't the most user friendly. The most common thing I do before editing my videos is manually reassemble the "long-form" videos in `Adobe Premiere Pro` or `Kdenlive`. This tool saves me a ton of time.

## How?

This tool combines multi-chapter videos using the [`ffmpeg concat demuxer`](https://stackoverflow.com/a/11175851) and renames single chapter videos so the filenames can be parsed more easily. You can disable the single chapter renaming behavior with `--no-single-chapter-rename`. If a multi-chapter merge operation is done, a set of commands will be printed at the end to clean up the original source directory. These commands are destructive, and have to be run manually.

All output files will have the form: `GoPro_{video_number}.MP4`.

## Installation

This package is available on [`crates.io`](https://crates.io/crates/gopro-chaptered-video-assembler). 

```bash
$ cargo install gopro-chaptered-video-assembler
```

## Example Usage

```bash
$ gopro-chaptered-video-assembler --input "/media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/" --output combined
╔═══════════════════════════════════════╗
║ gopro-chaptered-video-assembler 0.4.0 ║
╚═══════════════════════════════════════╝
01:49:33 [INFO] combined directory does not exist, attempting to create it now...
01:49:33 [INFO] /media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure/combined directory exists, using it...
01:49:33 [INFO] Found 39 files in directory: /media/alichtman/Extreme SSD/Rock-Climbing/Gunks/High Exposure
01:49:33 [WARN] Failed to parse GoPro video file: combined is a directory
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

The `combined` directory will be created in the current directory, and all output files will be found there.

Additionally, paths with apostrophes (and theoretically other special characters) are supported:

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
