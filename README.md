# Word Freq Statistic 词频统计工具 📄

[![README English](https://img.shields.io/badge/README-English-blue)](https://github.com/GarthTB/word-freq-statistic/blob/master/_en.md)
[![用前必读 中文](https://img.shields.io/badge/用前必读-中文-red)](https://github.com/GarthTB/word-freq-statistic/blob/master/README.md)
[![开发语言 Rust](https://img.shields.io/badge/开发语言-Rust-brown)](https://www.rust-lang.org)
[![最新版本 0.2.1](https://img.shields.io/badge/最新版本-0.2.1-brightgreen)](https://github.com/GarthTB/word-freq-statistic/releases)
[![开源许可 Apache 2.0](https://img.shields.io/badge/开源许可-Apache%202.0-royalblue)](https://www.apache.org/licenses/LICENSE-2.0)

## 简介

Word Freq Statistic 是一个高性能的词频统计工具，针对中文文本进行特定词长的盲分词处理并统计词频。

## 特点

- **高性能**：采用 Rust 语言编写，利用多线程并发处理，可在1分钟内统计10亿字语料的2字词。
- **小体积**：大小仅有 2MB 左右，无需安装依赖。
- **自定义字符筛选**：支持使用字符范围（配合额外字符）或正则表达式来筛选要纳入的字符。
- **TOML 配置文件**：通过 TOML 文件配置输入参数，如文件路径、词长、匹配模式等。

## 安装

### 下载

下载最新版本的 Word Freq Statistic 压缩包，解压后即可运行。

### 编译

如果需要编译源码，请确保安装了 Rust 环境，然后执行以下命令：

```
cargo build --release
```

编译完成后，可在 `target/release` 目录下找到可执行文件 `word_freq_statistic`。

## 使用

1. 准备待统计的语料文本文件，并将其放在程序所在目录下。
2. 编辑配置文件 `config.toml` ，配置输入参数。
3. 运行程序：

```
./word_freq_statistic
```

程序会根据配置文件中的参数，统计语料文本中每个词的词频，并输出为指定格式的文本文件。

### 配置文件

配置文件采用 TOML 格式，必须位于程序所在目录下，文件名固定为 `config.toml`，必须包含且仅包含以下9个参数：

- `input_filename`：输入语料文件名，应位于程序所在目录。
- `output_filename`：输出结果文件名，将输出到程序所在目录；若文件已存在，则覆盖。
- `word_length`：词的字数。
- `freq_threshold`：词频阈值：低于此值的词将被忽略。
- `use_regex`：字符筛选方式：false则使用UTF-8值范围和额外字符，true则使用正则表达式。
- `lower_limit`：UTF-8值范围下限：19968即\u4e00，即"一"字，会被包含。
- `upper_limit`：UTF-8值范围上限：40959即\u9fff，即"鿿"字，会被包含。
- `extra_chars`：UTF-8值范围外的额外字符，如逗号、句号、空格、生僻字等。
- `regex`：正则表达式；若 `use_regex = false` ，则忽略此项。

示例配置文件：

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

## 性能测试

测试环境：Intel(R) Core(TM) i5-12500H 2.50 GHz / 16GB RAM / Windows 11 26100.3915

测试结果：

|                         语料来源                         |   有效字数    | 词长 | 阈值 |          字符筛选方式          | 第一轮统计用时(秒) | 第二轮统计用时(秒) | 总用时(秒) |
|:----------------------------------------------------:|:---------:|:--:|:--:|:------------------------:|:----------:|:----------:|:------:|
|                          微博                          | 118642377 | 2  | 10 |    19968-40959，无额外字符     |    2.8     |    2.5     |  5.6   |
|                          微博                          | 118642377 | 2  | 10 |  正则表达式"[\u4e00-\u9fff]"  |    5.4     |    5.5     |  11.2  |
|                          微博                          | 118642377 | 4  | 10 |   19968-40959，额外字符"，。"   |    8.4     |    2.6     |  16.9  |
|                          微博                          | 118642377 | 4  | 10 | 正则表达式"[，。\u4e00-\u9fff]" |    11.8    |    5.6     |  23.3  |
| [万卷1.0](https://github.com/opendatalab/WanJuan1.0)网文 | 999435429 | 2  | 10 |    19968-40959，无额外字符     |    20.9    |    20.7    |  42.4  |

## 注意

- 程序进行针对中文文本的盲分词处理，即只考虑词的字数，不考虑词的词性、语义等。
- 语料文本应为UTF-8编码。程序不支持GBK、GB2312等编码。

## 更新日志

### v0.2.1 (2025-05-15)

- 修复：无法统计行尾残留词的问题
- 优化：将载入配置文件排除在总计时长之外

### v0.2.0 (2025-05-14)

- 新增：全局计时及两轮统计的分别计时
- 新增：显示筛选条件、总有效字数等信息
- 优化：提升词频阈值的筛选性能
- 优化：结尾直接退出（现为纯TOML输入，无任何交互）

### v0.1.0 (2025-05-10)

- 初始版本。
