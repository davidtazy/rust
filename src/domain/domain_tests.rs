mod tests {
    use crate::domain::Book;

    #[test]
    fn create_book() {
        let book = Book {
            id: 1,
            title: "Rust Programming".to_string(),
            year: 2025,
        };
        assert_eq!(book.id, 1);
        assert_eq!(book.title, "Rust Programming");
        assert_eq!(book.year, 2025);
    }
}
