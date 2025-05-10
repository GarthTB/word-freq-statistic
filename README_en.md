# Word Freq Statistic 📄

[![README English](https://img.shields.io/badge/README-English-blue)](https://github.com/GarthTB/word-freq-statistic/blob/master/_en.md)
[![用前必读 中文](https://img.shields.io/badge/用前必读-中文-red)](https://github.com/GarthTB/word-freq-statistic/blob/master/README.md)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-brown)](https://www.rust-lang.org)
[![Latest Release 0.2.0](https://img.shields.io/badge/Latest%20Release-0.2.0-brightgreen)](https://github.com/GarthTB/word-freq-statistic/releases)
[![License Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-royalblue)](https://www.apache.org/licenses/LICENSE-2.0)

## Introduction

Word Frequency Statistics is a high-performance application designed for blind word segmentation and frequency analysis of Chinese text based on specified word lengths.

## Features

- **High Performance**: Built with Rust and multithreaded concurrency, capable of processing 2-character words in 1 billion characters of text within 1 minute.
- **Compact Size**: Approximately 2MB in size, with no external dependencies.
- **Custom Matching Modes**: Supports character ranges (with optional extra characters) or regular expressions to filter valid characters.
- **TOML Configuration**: Configure input parameters via a TOML file, including file paths, word length, matching patterns, and more.

## Installation

### Download

Download the latest release package, extract it, and run the executable directly.

### Build from Source

Ensure Rust is installed, then compile using:

```
cargo build --release
```

The executable will be located at `target/release/word_freq_statistic`.

## Usage

1. Prepare a text corpus file and place it in the program directory.
2. Edit the config.toml file to configure parameters.
3. Run the program:

```
./word_freq_statistic
```

The tool will generate a frequency statistics file based on the configuration.

## Configuration File

The TOML configuration file (config.toml) must be located in the program directory and include **exactly** the following 9 parameters:

- `input_filename`: Input corpus filename (must be in the program directory).
- `output_filename`: Output result filename (will be output to the program directory, overwriting if it exists).
- `word_length`: Character length of words to be analyzed.
- `freq_threshold`: Frequency threshold: words with frequency below this value will be ignored.
- `use_regex`: Character filtering mode: false for UTF-8 value range and extra characters, true for regular expressions.
- `lower_limit`: UTF-8 value range lower limit: 19968 for "一" character, included.
- `upper_limit`: UTF-8 value range upper limit: 40959 for "鿿" character, included.
- `extra_chars`: Extra characters outside the UTF-8 value range, such as commas, periods, spaces, and rare characters.
- `regex`: Regular expression; ignored if `use_regex = false`.

Example:

```
input_filename = "input.txt"
output_filename = "input_statistics.txt"
word_length = 2
freq_threshold = 10
use_regex = false
lower_limit = 19968
upper_limit = 40959
extra_chars = ""
regex = "[\u4e00-\u9fff]"
```

## Performance Benchmarks

Test Environment: Intel(R) Core(TM) i5-12500H 2.50 GHz / 16GB RAM / Windows 11 26100.3915

Results:

- 2-character words in 1 billion characters (no regex, no extra chars, threshold=10): ~57 seconds.
- 4-character words in 120 million characters (extra char "，", threshold=10): ~23 seconds.
- 2-character words in 120 million characters (regex "[，\u4e00-\u9fff]", threshold=10): ~14 seconds.

## Notes

- The tool performs blind segmentation based solely on word length, ignoring semantics or part-of-speech.
- Corpus files must be UTF-8 encoded. GBK/GB2312 encodings are unsupported.

## Changelog

### v0.2.0 (2025-05-14)

- Added timings for each step of the calculation and the whole process.
- Added display of the filtering conditions and the total number of valid characters.
- Optimized the performance of filtering with frequency threshold.
- Fixed the program to exit directly when the end of the file is reached. (Now it is a pure TOML input, without any interactive input.)

### v0.1.0 (2025-05-10)

- Initial release.
