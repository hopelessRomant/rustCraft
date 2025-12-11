#[allow(dead_code)]
mod thread;

#[allow(dead_code)]
mod channel;

#[allow(dead_code)]
mod sharing;

fn main() {
    // thread::thread_test();
    // thread::move_ownership();

    // channel::sample_channel();
    // channel::channel_iter();
    // channel::mpsc();

    // sharing::sample_mutx();
    sharing::arc();

}
