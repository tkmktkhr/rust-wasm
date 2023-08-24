pub mod library {
  #[derive(Debug)]
  pub struct Library {
    pub books: Vec<Book>,
  }

  #[derive(Debug)]
  pub struct Book {
    title: String,
    year: u16,
  }

  impl Book {
    pub fn new(title: &str, year: u16) -> Book {
      Book {
        title: String::from(title),
        year,
      }
    }
  }

  impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} ({})", self.title, self.year)
    }
  }

  impl Library {
    pub fn new() -> Self {
      println! {"{:?}", String::from("test")};
      Library { books: vec![] }
    }

    pub fn get_len(&self) -> usize {
      //  todo!("Return the length of `self.books`")
      self.books.len()
    }

    pub fn get_is_empty(&self) -> bool {
      // todo!("Return `true` if `self.books` is empty")
      self.books.is_empty()
    }

    pub fn add_book(&mut self, book: Book) {
      println!("2: {:p}", self);
      self.books.push(book)
    }

    pub fn print_books(&self) -> () {
      for (i, book) in self.books.iter().enumerate() {
        println!("No{}: title [{}], year [{}]", i, book.title, book.year);
      }
    }

    pub fn get_oldest_book(&self) -> Option<&Book> {
      //  todo!("Return a reference to the oldest book (if any)")
      let oldest = self.books.iter().min_by(|a, b| a.year.cmp(&b.year));
      // let mut oldest = None;
      // for book in self.books.iter() {
      //   if oldest.is_none() || book.year < oldest.unwrap().year {
      //     oldest = Some(book);
      //   }
      // }
      oldest
    }
  }
}

#[test]
fn test_get_is_empty() {
  use super::storing_book::library::{Library, Book};

  let mut library = library::Library::new();
  assert_eq!(library.get_is_empty(), true);
  library.add_book(Book::new("test_book", 2020));
  assert_eq!(library.get_is_empty(), false);
}