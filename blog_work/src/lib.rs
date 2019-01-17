pub struct PostV1 {
    state: Option<Box<StateV1>>,
    content: String,
}

impl PostV1 {
    pub fn new() -> PostV1 {
        PostV1 {
            state: Some(Box::new(DraftV1 {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // If there were more of these type methods
    // looking into using macros, might be useful
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait StateV1 {
    fn request_review(self: Box<Self>) -> Box<StateV1>;
    fn approve(self: Box<Self>) -> Box<StateV1>;
    fn content<'a>(&self, post: &'a PostV1) -> &'a str {
        ""
    }
}

struct DraftV1 {}

impl StateV1 for DraftV1 {
    fn request_review(self: Box<Self>) -> Box<StateV1> {
        Box::new(PendingReviewV1 {})
    }
    fn approve(self: Box<Self>) -> Box<StateV1> {
        self
    }
    // could add content method where we decorate with "DRAFT"
}

struct PendingReviewV1 {}

impl StateV1 for PendingReviewV1 {
    fn request_review(self: Box<Self>) -> Box<StateV1> {
        self
    }
    fn approve(self: Box<Self>) -> Box<StateV1> {
        Box::new(PublishedV1 {})
    }
}

struct PublishedV1 {}

impl StateV1 for PublishedV1 {
    fn request_review(self: Box<Self>) -> Box<StateV1> {
        self
    }
    fn approve(self: Box<Self>) -> Box<StateV1> {
        self
    }
    fn content<'a>(&self, post: &'a PostV1) -> &'a str {
        &post.content
    }
}

// Encoding States and Behavior as Types
pub struct DraftPostV2 {
    content: String,
}

pub struct PostV2 {
    content: String,
}

pub struct PendingReviewPostV2 {
    content: String,
}

impl PostV2 {
    pub fn new() -> DraftPostV2 {
        DraftPostV2 {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPostV2 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPostV2 {
        PendingReviewPostV2 {
            content: self.content,
        }
    }
}

impl PendingReviewPostV2 {
    pub fn approve(self) -> PostV2 {
        PostV2 {
            content: self.content,
        }
    }
}
