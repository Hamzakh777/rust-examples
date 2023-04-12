## Smart Pointers
The most common kind of pointers in Rust is a reference which is indicated by the `&` symbol and borrow the value they point to.

`Smart Pointers` are data structures that act like a pointer (Struct), but have additional metadata and capabilities.
Examples of Smart pointers include: **reference counting**.
`Smart Pointers` in many cases own the data they point to. For example `Vec` and `String` are smart pointers.

`Smart Pointers` are usually implemented using Structs, they implement the `Deref` and `Drop` traits.

Most common pointers in the standard library:
- `Box<T>`: for allocating values on the heap
- `Rc<T>`: a reference counting type that enables multiple ownership.
- `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time.

**interior mutability pattern**: Immutable type exposes an API for mutating an interior value.
**reference cyles**: they can leak memory and needs to be prevented.