use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn somthing_great_async_function1() -> i32 {
    let ans = async_add(2, 3).await;
    println!("{}", ans);
    ans
}

async fn somthing_great_async_function2() -> i32 {
    let ans1 = async_add(1, 2).await;
    let ans2 = async_add(2, 3).await;
    let ans3 = async_add(3, 4).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

fn main() {
    executor::block_on(somthing_great_async_function1());
    executor::block_on(somthing_great_async_function2());
}