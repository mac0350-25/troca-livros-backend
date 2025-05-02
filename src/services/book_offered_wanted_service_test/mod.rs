pub mod add_book_to_offered_test;
pub mod remove_book_from_offered_test;
pub mod add_book_to_wanted_test;
pub mod remove_book_from_wanted_test;

pub use crate::services::test_mocks::{
    MockBookRepository, MockBooksOfferedRepository, MockBooksWantedRepository, 
    MockGoogleBookService, create_test_book_with_id
};