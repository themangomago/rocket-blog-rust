pub struct Post {
    uuid: String,
    author_id: u32,
    title: String,
    summary: String,
    body: String,
}

impl Post {
    fn new(uuid: String, author_id: u32, title: String, summary: String, body: String) -> Post {
        Post {
            uuid,
            author_id,
            title,
            summary,
            body,
        }
    }

    fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    fn get_author_id(&self) -> u32 {
        self.author_id
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_summary(&self) -> String {
        self.summary.clone()
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }
}
