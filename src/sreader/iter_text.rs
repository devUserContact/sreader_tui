use std::fs;

#[derive(Debug)]
pub struct IterText {}

struct BookSlice {
    array: Vec<String>,
    length: usize,
}

fn load_book() {
    let book: String =
        fs::read_to_string("assets/book/lewisCarroll_alicesAdventuresInWonderland.txt")
            .expect("failed to read file");
    let book_words: Vec<String> = book.split(" ").map(|s| s.to_string()).collect();
    BookSlice {
        array: book_words.clone(),
        length: book_words.len(),
    };
}
