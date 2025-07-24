//! #函数操作 模块 ( repl_function.rs )

//! ##功能

//! 封装函数操作

//! ##作者

//! songjiaqicode

//! ##初始化日期

//! 2025.7.24

/* 引入标准库 */



/* 引入私有库 */



/* 内部操作 */

pub fn run(){
    let cli = "\x1b[31m命令格式\x1b[0m:\nclick-dev \x1b[38;5;29m[编程语言]\x1b[0m \x1b[38;5;220m[ide]\x1b[0m\n";
    let lang = "\x1b[38;5;29m[编程语言]\x1b[0m:\nc# c c++ go html-css-js java python rust\n";
    let ide = "\x1b[38;5;220m[ide]\x1b[0m:\nvscode IntelliJ";
    println!("{}",&format!("{cli}{lang}{ide}"));
}