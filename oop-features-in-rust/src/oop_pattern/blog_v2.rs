// This is a state pattern but without having to use objects the same way as OOP
// Instead of having different state structs we will have different Post struct
// where its Struct describes a state
// This way, we wont have to implement all functions every-time we extend the trait on the State struct

// The Post struct won't have the State field anymore because we are moving the encoding of the state
// to the types of the structs.

// ===> Implementing Transitions as Transformations into Different Types

struct Post {
    content: String,
}

struct PostDraft {
    content: String,
}

impl Post {
    pub fn new() -> PostDraft {
        PostDraft {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl PostDraft {
    pub fn add_content(&mut self, text: &str) {
        self.content.push_str(text);
    }
}