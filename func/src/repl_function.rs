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
    let cli = "命令格式:\nclick-dev [编程语言] [ide]\n";
    let lang = "[编程语言]:\nc# c c++ go html-css-js java python rust\n";
    let ide = "[ide]:\nvscode IntelliJ";
    println!("{}",&format!("{cli}{lang}{ide}"));
}