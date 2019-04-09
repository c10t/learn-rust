pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // We call the `as_ref` method on the Option
        // because we want a reference to the value inside the Option rather than ownership of the value.
        self.state.as_ref()
        // Because state is an `Option<Box<dyn State>>`, when we call `as_ref`, an `Option<&Box<dyn State>>` is returned.
        // If we didn’t call as_ref, we would get an error
        // because we can’t move state out of the borrowed &self of the function parameter.
            .unwrap()
        // We then call the unwrap method, which we know will never panic,
        // because we know the methods on Post ensure that state will always contain a Some value when those methods are done.
            .content(&self)
        // At this point, when we call content on the &Box<dyn State>, 
        // deref coercion will take effect on the & and the Box
        // so the content method will ultimately be called on the type that implements the State trait.
        // That means we need to add content to the State trait definition,
        // and that is where we’ll put the logic for what content to return depending on which state we have,
    }

    // To consume the old state,
    // the request_review method needs to take ownership of the state value.
    // This is where the Option in the state field of Post comes in:
    pub fn request_review(&mut self) {
        // we call the take method to take the `Some` value out of the state field
        // and leave a None in its place,
        // because Rust doesn’t let us have unpopulated fields in structs.
        // This lets us move the state value out of Post rather than borrowing it.
        if let Some(s) = self.state.take() {
            // Then we’ll set the post’s state value to the result of this operation.
            self.state = Some(s.request_review())
            // We need to set state to `None` temporarily rather than setting it directly
            // with code like `self.state = self.state.request_review();` to get ownership of the state value.
            // This ensures Post can’t use the old state value after we’ve transformed it into a new state.
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // This syntax means the method is only valid
    // when called on a Box holding the type.
    // This syntax takes ownership of Box<Self>, invalidating the old state
    // so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
