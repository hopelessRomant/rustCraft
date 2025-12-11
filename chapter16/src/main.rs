#[allow(dead_code)]
mod thread;

#[allow(dead_code)]
mod channel;

fn main() {
    // thread::thread_test();
    // thread::move_ownership();

    // channel::sample_channel();
    // channel::channel_iter();
    channel::mpsc();

}
