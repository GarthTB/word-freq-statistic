use crate::config::Config;
use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub(crate) fn run() -> Result<f32, anyhow::Error> {
    println!("载入配置文件...");
    let config = Config::load()?;
    let word_len = config.word_length;
    let is_valid_char = config.get_judge_char_func()?;
    let time = Instant::now();
    println!("载入成功！开始计时！即将统计 {word_len} 字词...");

    println!("第一轮统计...");
    let time1 = Instant::now();
    let total_count = AtomicUsize::new(0);
    let comb_freq: DashMap<String, AtomicUsize> = DashMap::with_capacity(1 << 16);
    config.par_for_each_input_line(|line| {
        let mut heads: VecDeque<usize> = VecDeque::with_capacity(word_len);
        let mut local_count: usize = 0;
        for (i, c) in line.char_indices() {
            if is_valid_char(c) {
                local_count += 1;
                heads.push_back(i);
                if heads.len() == word_len {
                    let head = heads.pop_front().unwrap();
                    let tail = i + c.len_utf8();
                    record(&comb_freq, &line[head..tail]);
                }
            } else {
                heads.clear();
            }
        }
        total_count.fetch_add(local_count, Ordering::Relaxed);
    })?;
    println!("统计完成！语料有效字符总数：{}", total_count.into_inner());
    println!("统计用时：{:.6} s", time1.elapsed().as_secs_f32());

    println!("使用阈值 {} 初步筛选...", config.freq_threshold);
    comb_freq.retain(|_, freq| freq.load(Ordering::Relaxed) >= config.freq_threshold);
    comb_freq.shrink_to_fit();
    println!("筛选完成！有效字符组合（潜在词）总数：{}", comb_freq.len());

    println!("第二轮统计...");
    let time2 = Instant::now();
    let word_freq: DashMap<String, AtomicUsize> = DashMap::with_capacity(comb_freq.len());
    config.par_for_each_input_line(|line| {
        let mut heads: VecDeque<usize> = VecDeque::with_capacity(word_len);
        for (i, c) in line.char_indices() {
            if is_valid_char(c) {
                heads.push_back(i);
                if heads.len() == 2 * word_len - 1 {
                    let tail = i + c.len_utf8();
                    count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_len);
                }
            } else {
                if heads.len() >= word_len {
                    let tail = i + c.len_utf8();
                    count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_len);
                }
                heads.clear();
            }
        }
        if heads.len() >= word_len {
            let tail = line.len();
            count_word(&word_freq, &comb_freq, &line, &mut heads, tail, word_len);
        }
    })?;
    println!("统计完成！用时：{:.6} s", time2.elapsed().as_secs_f32());

    println!("使用阈值 {} 筛选并排序词...", config.freq_threshold);
    let mut result: Vec<(String, usize)> = word_freq
        .into_iter()
        .par_bridge()
        .filter_map(|(word, freq)| {
            let freq = freq.into_inner();
            if freq >= config.freq_threshold {
                Some((word, freq))
            } else {
                None
            }
        })
        .collect();
    result.sort_by(|(_, a_freq), (_, b_freq)| b_freq.cmp(a_freq));
    println!("筛选并排序完成！词的总数：{}", result.len());

    println!("输出结果...");
    let report: Vec<String> = result
        .into_iter()
        .map(|(word, freq)| format!("{word}\t{freq}"))
        .collect();
    config.output_bytes(report.join("\n").as_bytes())?;
    println!("输出完成！");

    Ok(time.elapsed().as_secs_f32())
}

fn record(map: &DashMap<String, AtomicUsize>, key: &str) {
    match map.get_mut(key) {
        Some(freq) => _ = freq.fetch_add(1, Ordering::Relaxed),
        None => _ = map.insert(key.to_string(), AtomicUsize::new(1)),
    }
}

fn count_word(
    word_freq: &DashMap<String, AtomicUsize>,
    comb_freq: &DashMap<String, AtomicUsize>,
    line: &str,
    heads: &mut VecDeque<usize>,
    window_tail: usize,
    word_length: usize,
) {
    let mut max_word: &str = "";
    let mut max_freq: usize = 0;
    while heads.len() >= word_length {
        let comb_head = heads.pop_front().unwrap();
        let comb_tail = if heads.len() >= word_length {
            heads[word_length - 1]
        } else {
            window_tail
        };
        if let Some(freq) = comb_freq.get(&line[comb_head..comb_tail]) {
            let freq = freq.load(Ordering::Relaxed);
            if freq > max_freq {
                max_word = &line[comb_head..comb_tail];
                max_freq = freq;
            }
        }
    }
    if max_freq > 0 {
        record(word_freq, max_word);
    }
}
