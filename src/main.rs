use func;

#[tokio::main]
async fn main(){
    /* 调用接口 */

    cmd::repl_function::run().await;

    func::repl_function::run();
}