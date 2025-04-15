#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{adapters::output::JsonBookRepository, domain::{Book, BookRepository}};

    #[test]
    fn save_and_retrieve_book() {
        let test_db = "test_books.json".to_string();
        let _ = std::fs::remove_file(&test_db);

        let mut repo = JsonBookRepository::new(test_db.clone());
        let book = Book {
            id: 1,
            title: "Rust in Action".to_string(),
            year: 2023,
        };

        repo.save(book.clone());
        let retrieved = repo.find_by_id(1).unwrap();

        assert_eq!(retrieved.title, "Rust in Action");
        assert_eq!(retrieved.year, 2023);

        let _ = fs::remove_file(&test_db);
    }
}
