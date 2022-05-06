struct Post {
    content: String
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

struct DraftPost {
    content: String
}

impl DraftPost {
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> ReviewedPost {
        ReviewedPost { content: self.content }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

struct ReviewedPost {
    content: String
}

impl ReviewedPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }

    pub fn reject(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_request_review() {
        let mut post = Post::new();
        post.add_text("hello world");

        let post = post.request_review();
        let post = post.reject();
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!(String::from("hello world"), post.content());
    }
}