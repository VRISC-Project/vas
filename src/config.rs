use clap::Parser;

/// vrisc架构汇编器
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// 输出文件
    #[arg(short, long)]
    pub output: String,

    /// 输入文件
    #[arg(short, long)]
    pub input: String,

    /// 输出格式[elf64|sel|raw]
    #[arg(short, long)]
    pub format: String,
}
