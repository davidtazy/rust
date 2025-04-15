mod domain;
mod application;
mod adapters;

use adapters::input::{json_input_adapter, BookAction};
use adapters::output::json_book_repository::JsonBookRepository;
use application::book_service::BookService;

fn main() {
    let actions = json_input_adapter::load_actions_from_json("input_actions.json");

    let repo = JsonBookRepository::new("books_db.json".to_string());
    let mut service = BookService { repository: repo };

    for action in actions {
        match action {
            BookAction::Create { id, title, year } => service.create_book(id, title, year),
            BookAction::Update { id, title, year } => service.update_book(id, title, year),
            BookAction::Delete { id } => service.delete_book(id),
        }
    }
}
