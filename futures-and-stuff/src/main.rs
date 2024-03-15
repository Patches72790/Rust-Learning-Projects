use std::{
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{
        Context,
        Poll::{self, Pending, Ready},
        Waker,
    },
    thread::{sleep, spawn},
    time::Duration,
};

use futures::{
    future::{ready, BoxFuture, Future},
    select,
    task::{waker_ref, ArcWake},
    FutureExt,
};
use tracing::info;

async fn _test_async_select() {
    let mut f1 = ready(5);
    let mut f2 = ready(5);
    let mut total = 0;

    loop {
        select! {
            a = f1 => total += a,
            b = f2 => total += b,
            complete => break,
            default => unreachable!("uh oh!")
        }
    }

    assert_eq!(total, 10)
}

fn main() {
    tracing_subscriber::fmt().init();
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        info!("howdy!");

        TimerFuture::new(Duration::from_secs(5)).await;
        info!("done!");
    });

    executor.run();
}

struct TimerFuture {
    state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            complete: false,
            waker: None,
        }));

        let cloned = state.clone();

        spawn(move || {
            sleep(duration);
            let mut state = cloned.lock().unwrap();
            state.complete = true;

            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        Self { state }
    }
}

#[derive(Clone)]
pub struct SharedState {
    complete: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.complete {
            Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Pending
        }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();

        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("Too many tasks")
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();

        arc_self.task_sender.send(cloned).expect("Too many tasks")
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let mut cx = Context::from_waker(&waker);

                if future.as_mut().poll(&mut cx).is_pending() {
                    *future_slot = Some(future)
                }
            }
        }
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;

    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}
