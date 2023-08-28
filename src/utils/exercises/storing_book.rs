pub mod library {
  #[derive(Debug)]
  pub struct Library {
    pub books: Vec<Book>,
  }

  #[derive(Debug, PartialEq, Clone)]
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
      self.books.push(book);
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

#[cfg(test)]
mod library_tests {
  use super::library::{self, Book, Library};
  #[test]
  fn test_get_is_empty() {
    let mut library: Library = library::Library::new();
    assert_eq!(library.get_is_empty(), true);
    library.add_book(Book::new("test_book", 2020));
    assert_eq!(library.get_is_empty(), false);
  }

  #[test]
  fn test_add_books() {
    let mut library = library::Library::new();
    let new_book1 = Book::new("book1", 1990);
    let expected = new_book1.clone();

    library.add_book(new_book1);

    let first_book = &library.books[0];
    assert_eq!(first_book, &expected);
  }

  #[test]
  fn test_get_oldest_book() {
    let mut library = library::Library::new();
    library.add_book(Book::new("first book", 1990));
    library.add_book(Book::new("second book", 1995));
    library.add_book(Book::new("third book", 2000));
    library.add_book(Book::new("third book", 1990));

    let oldest_book = library.get_oldest_book().unwrap();
    let expected = Book::new("first book", 1990);
    assert_eq!(library.get_len(), 4);
    assert_eq!(oldest_book, &expected)
  }
}
