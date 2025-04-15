use crate::domain::{Book, BookRepository};
use serde_json;
use std::fs;
use std::collections::HashMap;

pub struct JsonBookRepository {
    pub path: String,
    books: HashMap<u32, Book>,
}

impl JsonBookRepository {
    pub fn new(path: String) -> Self {
        let books = fs::read_to_string(&path)
            .map(|content| serde_json::from_str(&content).unwrap_or_default())
            .unwrap_or_default();
        Self { path, books }
    }

    fn persist(&self) {
        let json = serde_json::to_string_pretty(&self.books).unwrap();
        fs::write(&self.path, json).unwrap();
    }
}

impl BookRepository for JsonBookRepository {
    fn save(&mut self, book: Book) {
        self.books.insert(book.id, book);
        self.persist();
    }

    fn update(&mut self, book: Book) {
        self.books.insert(book.id, book);
        self.persist();
    }

    fn delete(&mut self, id: u32) {
        self.books.remove(&id);
        self.persist();
    }

    fn find_by_id(&self, id: u32) -> Option<Book> {
        self.books.get(&id).cloned()
    }
}
