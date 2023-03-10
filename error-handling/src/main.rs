mod unrecoverable_errors;

fn main() {
    // Rust groups errors into two groups:
    // - Recoverable: report and retry 
    // - Unrecoverable: symptom of bugs
    // Rust doesn't have exceptions, instead it has the Type `Result<T, E>` for recoverable errors
    // and `!panic` macro that stops executions
    unrecoverable_errors::panic();
    unrecoverable_errors::result();
}
