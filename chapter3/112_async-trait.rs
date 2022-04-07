use async_trait::async_trait;
use futures::executor;

#[async_trait]
pub trait AsyncTrait {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait for Runner {
    async fn f(&self) {
        println!("Hello, async-trait");
    }
}

fn main() {
    let runner = Runner {};
    executor::block_on(runner.f());
}