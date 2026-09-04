use clap::{CommandFactory, Parser};

use fisource::{commands, scan};

#[derive(Parser)]
#[command(name = "fisource", version, about = "脱敏的、agent 友好的凭据管理系统")]
struct Cli {
    /// env 文件名（source 模式）
    file: Option<String>,

    /// 列举所有 key（脱敏）
    #[arg(long)]
    keys: bool,

    /// 列出所有 env 文件及描述和 key（脱敏）
    #[arg(long)]
    list: bool,

    /// 展示 env 校验矩阵（脱敏）
    #[arg(long)]
    show: bool,

    /// fzf 候选列表（内部使用）
    #[arg(long, hide = true)]
    fzf_list: bool,

    /// fzf 预览指定文件（内部使用）
    #[arg(long, hide = true, value_name = "FILE")]
    fzf_preview: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.keys {
        print!("{}", commands::cmd_keys(&scan::scan_all()));
    } else if cli.list {
        print!("{}", commands::cmd_list(&scan::scan_all()));
    } else if cli.show {
        print!("{}", commands::cmd_show(&scan::scan_all()));
    } else if cli.fzf_list {
        print!("{}", commands::cmd_fzf_list(&scan::scan_all()));
    } else if let Some(file) = cli.fzf_preview {
        if let Some(f) = scan::load_one(&file) {
            print!("{}", commands::cmd_fzf_preview(&f));
        }
    } else if let Some(file) = cli.file {
        match scan::load_one(&file) {
            Some(f) => print!("{}", commands::cmd_source(&f)),
            None => {
                eprintln!("fisource: 未找到 env 文件: {}", file);
                std::process::exit(1);
            }
        }
    } else {
        Cli::command().print_help().ok();
        std::process::exit(0);
    }
}
