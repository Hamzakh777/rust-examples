mod threads;
mod message_passing;
mod mutex;
mod sync_send_traits;

fn main() {
    // threads::run();
    // message_passing::run();
    mutex::run();
    sync_send_traits::run();
}
