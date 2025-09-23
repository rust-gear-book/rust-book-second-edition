use async_trait::async_trait;

#[async_trait]
trait Runnable {
    async fn run(&self);
}

fn prepare_executor() -> Box<dyn Runnable> {
    todo!()
}

fn main() {}
