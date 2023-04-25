// the advantages of the State pattern is that its State is responsible for its own rules.

// using an enum we would need to have multiple match arms inside its Post method
// This can get quite messy if we have multiple states
// plus the logic is scattered in multiple places

// By using the state the pattern, adding a new state is a matter of creating a new struct 
// that implements the State trait. 
// Also, all the logic related to that state (when to move, what date to return etc) is 
// encapsulated into one Struct/Implementation

// The downside of the state pattern is that because the States handle the transition between 
// one another, some of these states are coupled to one another.
// Another downside is we have a lot of duplication.

// By implementing the state pattern exactly as it’s defined for object-oriented languages, 
// we’re not taking as full advantage of Rust’s strengths as we could.
// We can make invalid state changes into compile time errors

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    // `self: Box<Self>` means this syntax is only valid on when called on a `Box` holding type
    // It takes ownership of `Box<Self>` invalidating the current of the `Post` so it can transform to another one,
    // thereby consuming the old state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::default())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approvals: u8
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // returns itself because when a Post is in PendingReview,
        // and we request a review it should stay in PendingReview state.
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.as_ref().approvals == 2 {
            Box::new(Published {})
        } else {
            self.as_mut().approvals += 1;
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
impl Default for PendingReview {
    fn default() -> Self {
        Self {
            approvals: 0
        }
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


pub fn run() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}