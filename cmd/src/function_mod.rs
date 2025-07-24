//! #函数操作 模块 ( function_mod.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.24

/* 引入标准库 */

use std::io::stdin;
use tokio::process::Command;

/* 选择枚举 */
pub enum Select{
    V,/* 版本号 */
    Code,/* 代码仓库 */
    Csharp,/* C# */
    C,/* c */
    Cplusplus,/* c++ */
    Go,/* go */
    HtmlCssJs,/* html-css-js */
    Java,/* java */
    Python,/* python */
    Rust/* rust */
}

/* 匹配枚举值 */
pub async fn select(cli:Select){
    match cli {
        Select::V => { println!("0.1.0"); off() }
        Select::Code => { if let Ok(_) = jump("https://gitcode.com/songjiaqicode/click-dev").await{ off() }else { err() } }
        Select::Csharp => {
            select_cmd("是否安装 c# [y/n]");
            #[cfg(target_os = "windows")] let _ = cmd(r#"curl -L aka.ms/vs/17/release/vs_community.exe -o /tmp/vs.exe && /tmp/vs.exe --passive --add Microsoft.VisualStudio.Workload.ManagedDesktop --installPath "$(df -h | awk '$6 !~ /^\/c$/ && $1 ~ /^\/[a-z]/ {print $6; exit}')/VS" --nocache --wait"#).await;
            #[cfg(target_os = "linux")] println!("暂不支持linux");
            off()
        }
        Select::C => { select_cmd("是否安装 c [y/n]"); off() }
        Select::Cplusplus => { select_cmd("是否安装 c++ [y/n]"); off() }
        Select::Go => { select_cmd("是否安装 go [y/n]"); off() }
        Select::HtmlCssJs => { select_cmd("是否安装 html-css-js [y/n]"); off() }
        Select::Java => { select_cmd("是否安装 java [y/n]"); off() }
        Select::Python => { select_cmd("是否安装 python [y/n]"); off() }
        Select::Rust => { select_cmd("是否安装 rust [y/n]"); off() }
    }
}

/* 通用跳转 */
pub async fn jump(https:&str) -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    cmd(&format!("xdg-open {}",https)).await?;

    #[cfg(target_os = "windows")]
    Command::new("cmd").args(["/C",https]).status().await?;

    Ok(())
}

/* 执行命令 */
async fn cmd(shell:&str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    Command::new("bash").args(["-c",shell]).status().await?;

    #[cfg(target_os = "windows")]
    Command::new(r#"C:\msys2\usr\bin\bash.exe"#).args(["-c",shell]).status().await?;

    Ok(())
}

/* 通用选择 */
fn select_cmd(pr:&str) {
    println!("{}",pr);
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    if buf.trim() == "y" { () }else { println!("操作已取消"); std::process::exit(0); }
}

/* 通用关闭 */
fn off(){ std::process::exit(0) }

/* 通用报错 */
fn err(){ println!("错误☠️"); std::process::exit(0) }