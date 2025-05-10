mod config;
mod core;

fn main() {
    println!(
        "欢迎使用 Word Freq Statistic 词频统计工具！\n\
        版本号：0.2.0 (2025-05-14)\n\
        作者：Garth TB <g-art-h@outlook.com>\n\
        仓库地址：https://github.com/GarthTB/word-freq-statistic"
    );
    match core::run() {
        Ok(t) => println!("程序成功执行完毕！总用时：{t} s"),
        Err(e) => println!("程序出错：{e:?}\n\n已中断！"),
    }
}
