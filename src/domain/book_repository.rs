use super::book::Book;

pub trait BookRepository {
    fn save(&mut self, book: Book);
    fn update(&mut self, book: Book);
    fn delete(&mut self, id: u32);
    fn find_by_id(&self, id: u32) -> Option<Book>;
}
