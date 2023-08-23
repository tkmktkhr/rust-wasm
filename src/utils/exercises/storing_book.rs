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
  }
}
