pub struct BookDomain {
    pub id: Option<String>,
    pub book_name: String,
    pub description: String,
    pub is_test: bool
}

pub fn validate_field(book: BookDomain)-> BookDomain {
    if book.book_name == "livro teste" {
        let book_field =  BookDomain {
            id : book.id,
            book_name: book.book_name,
            description: book.description,
            is_test: true,
        };

        return book_field
    }

    book
}