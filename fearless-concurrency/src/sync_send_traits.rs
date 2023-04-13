// Extensible concurrency with `Sync` and `Send` traits
// Almost all the concurrency features in Rust are implemented in the standard library not the language itself
// You can write your own concurrency options or use those created by others.

// Two concurrency concepts are embedded into the language `std::marker` and traits `Sync` and `Send`

// ==== Allowing ownership transfer between threads by using `Send`
// Almost all types in Rust are `Send`, there are some exceptions like Rc<T> and raw pointers
// Any type composed entirely of `Send` types is automatically marked as `Send` as well

// ==== Allowing access from multiple threads with `Sync`
// `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced
// from other threads. 
// Any type T is `Sync` if `&T` (an immutable reference to T) is `Send`
// Primitive types are `Sync` and types composed entirely of types that are `Sync` are also `Sync.
// The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads

// ==== Implementing Send and Sync Manually Is Unsafe
// check `The rustonomicon` for more information
// Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
// As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.