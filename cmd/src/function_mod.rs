//! #函数操作 模块 ( function_mod.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.24

/* 引入标准库 */

use tokio::process::Command;

/* 选择枚举 */
pub enum Select{
    V,/* 版本号 */
    C,/* 代码仓库 */
}

/* 匹配枚举值 */
pub async fn select(cli:Select){
    match cli {
        Select::V => { println!("0.1.0"); off() }
        Select::C => { if let Ok(_) = jump("https://gitcode.com/songjiaqicode/click-dev").await{ off() }else { err() } }
    }
}

/* 通用跳转 */
pub async fn jump(https:&str) -> Result<(), Box<dyn std::error::Error>> {
    /* 跳转代码仓库 */
    #[cfg(target_os = "linux")]
    cmd(&format!("xdg-open {}",https)).await?;

    #[cfg(target_os = "windows")]
    Command::new("cmd").args(["/C","start",https]).status().await?;

    Ok(())
}

/* 执行命令 */
async fn cmd(shell:&str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("bash").arg("-c").arg(shell).status().await?;
    Ok(())
}

/* 通用关闭 */
fn off(){ std::process::exit(0) }

/* 通用报错 */
fn err(){ println!("错误☠️"); std::process::exit(0) }