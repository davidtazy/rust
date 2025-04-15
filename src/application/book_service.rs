use crate::domain::{Book, BookRepository};

pub struct BookService<R: BookRepository> {
    pub repository: R,
}

impl<R: BookRepository> BookService<R> {
    pub fn create_book(&mut self, id: u32, title: String, year: u16) {
        let book = Book { id, title, year };
        self.repository.save(book);
    }

    pub fn update_book(&mut self, id: u32, title: String, year: u16) {
        let book = Book { id, title, year };
        self.repository.update(book);
    }

    pub fn delete_book(&mut self, id: u32) {
        self.repository.delete(id);
    }
}
