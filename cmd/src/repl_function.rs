//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.24

/* 引入标准库 */

use std::env::{args};

/* 引入私有库 */

/* 参数命令处理模块 */
#[path = "function_mod.rs"]
mod function_mod;

use function_mod::Select;

/* 内部操作 */

pub async fn run(){
    /* 获取命令行参数 */
    for i in args().skip(1) {
        match i.as_str() {
            "v" => { function_mod::select(Select::V).await }
            "c" => { function_mod::select(Select::C).await }
            _ => { println!("未定义的命令"); std::process::exit(0);}
        }
    }
}