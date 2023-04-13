mod threads;
mod message_passing;

fn main() {
    threads::run();
    message_passing::run();
}
