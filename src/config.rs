use anyhow::{Context, Error};
use rayon::prelude::*;
use std::collections::HashSet;
use std::env::current_exe;
use std::fs::{File, read_to_string, remove_file};
use std::io::{BufRead, BufReader, Write};

#[derive(serde::Deserialize)]
/// 从config.toml中读取的配置信息及其相关操作
pub(crate) struct Config {
    /// 输入语料文件名，应位于程序所在目录
    input_filename: String,
    /// 输出结果文件名，将输出到程序所在目录；若文件已存在，则覆盖
    output_filename: String,
    /// 词的字数
    pub(crate) word_length: usize,
    /// 词频阈值：低于此值的词将被忽略
    pub(crate) freq_threshold: usize,
    /// 字符筛选方式：false则使用UTF-8值范围和额外字符，true则使用正则表达式
    use_regex: bool,
    /// UTF-8值范围下限：19968即\u4e00，即"一"字，会被包含
    lower_limit: u32,
    /// UTF-8值范围上限：40959即\u9fff，即"鿿"字，会被包含
    upper_limit: u32,
    /// UTF-8值范围外的额外字符，如逗号、句号、空格、生僻字等
    extra_chars: String,
    /// 正则表达式；若use_regex为false，则忽略此项
    regex: String,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let config_path = exe_path.with_file_name("config.toml");
        let config_str = read_to_string(&config_path).context("无法读取配置文件")?;
        toml::from_str(&config_str).context("配置文件格式错误")
    }

    pub(crate) fn par_for_each_input_line(&self, op: impl Fn(String) + Sync) -> Result<(), Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let input_path = exe_path.with_file_name(&self.input_filename);
        let input_file = File::open(&input_path).context("无法打开输入语料文件")?;
        Ok(BufReader::new(input_file)
            .lines()
            .par_bridge()
            .for_each(|line| match line {
                Ok(line) => op(line),
                _ => {}
            }))
    }

    pub(crate) fn output_bytes(&self, bytes: &[u8]) -> Result<(), Error> {
        let exe_path = current_exe().context("无法获取程序路径")?;
        let output_path = exe_path.with_file_name(&self.output_filename);
        if output_path.exists() {
            println!("输出文件已存在，将覆盖");
            remove_file(&output_path).context("无法删除已存在的输出文件")?;
        }
        let mut output_file = File::create(&output_path).context("无法创建输出结果文件")?;
        output_file.write_all(bytes).context("无法写入输出结果文件")
    }

    pub(crate) fn get_judge_char_func(
        &self,
    ) -> Result<Box<dyn Fn(char) -> bool + Sync + '_>, Error> {
        if self.use_regex {
            let regex = regex::Regex::new(&self.regex).context("正则表达式格式错误")?;
            println!("字符筛选方式：正则表达式 \"{}\"", self.regex);
            Ok(Box::new(move |c: char| regex.is_match(&c.to_string())))
        } else {
            let extra_chars: HashSet<char> = self
                .extra_chars
                .chars()
                .filter(|c| (*c as u32) < self.lower_limit || (*c as u32) > self.upper_limit)
                .collect();
            if extra_chars.is_empty() {
                println!(
                    "字符筛选方式：UTF-8值范围 [{}, {}] （共 {} 个字符）",
                    self.lower_limit,
                    self.upper_limit,
                    self.upper_limit - self.lower_limit + 1
                );
                Ok(Box::new(|c: char| {
                    (c as u32) >= self.lower_limit && (c as u32) <= self.upper_limit
                }))
            } else {
                println!(
                    "字符筛选方式：UTF-8值范围 [{}, {}] 和 {} 个额外字符（共 {} 个字符）",
                    self.lower_limit,
                    self.upper_limit,
                    extra_chars.len(),
                    (self.upper_limit - self.lower_limit + 1) as usize + extra_chars.len()
                );
                Ok(Box::new(move |c: char| {
                    (c as u32) >= self.lower_limit && (c as u32) <= self.upper_limit
                        || extra_chars.contains(&c)
                }))
            }
        }
    }
}
