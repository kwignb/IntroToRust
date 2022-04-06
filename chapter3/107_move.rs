use futures::future::Future;
use futures::executor;

fn move_to_async_block() -> impl Future<Output = ()> {
    let ouside_variable = "this is outside".to_string();
    async move {
        println!("{}", ouside_variable);
    }
}

fn main() {
    executor::block_on(move_to_async_block());
}