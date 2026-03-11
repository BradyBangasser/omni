use std::future::Future;

#[derive(Clone, Copy, Debug)]
pub struct LocalExecuter;

impl<F> hyper::rt::Executor<F> for LocalExecuter
where
    F: Future + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn_local(fut);
    }
}
