pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    // ヒント: ステートオブジェクトに内容について変わる可能性のあるものの責任を持たせつつも、 Postを変更することには責任を持たせない。
    // つまり、 State の add_text には (&mut self.content, text) だけ渡す。
    pub fn add_text(&mut self, text: &str) {
        self.state
            .as_ref()
            .unwrap()
            .add_text(&mut self.content, text);
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        // take(): Takes the value out of the option, leaving a None in its place.
        // state の所有権を奪い、後から結果をいれる。
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
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn add_text(&self, _content: &mut String, _text: &str) {}
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
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
