pub trait Widget {
  /// Natural width of `self`.
  fn width(&self) -> usize;

  /// Draw the widget into a buffer.
  fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

  /// Draw the widget on standard output.
  fn draw(&self) {
    let mut buffer = String::new();
    self.draw_into(&mut buffer);
    println!("{buffer}");
  }
}

pub struct Label {
  label: String,
}

impl Label {
  fn new(label: &str) -> Label {
    Label {
      label: label.to_owned(), // Creates owned data from borrowed data, usually by cloning.
    }
  }
}

// fn add_widget(&mut self, widget: Box<dyn Widget>) {
// window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
impl Widget for Label {
  fn width(&self) -> usize {
    self
      .label
      .lines()
      .map(|line| line.chars().count())
      .max()
      .unwrap_or(0)
    // if label is just 1 line, label.len() should be fine.
  }

  fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
    println!("widget label: draw into");
    println!("{:^1$}", self.label, self.width()); // self.width must be 30.
                                                  // You can use named arguments in the format specifier by appending a `$`.
                                                  // println!("{number:0>width$}", number=1, width=5);
    writeln!(buffer, "{:^1$}", self.label, self.width()).unwrap();
  }
}

pub struct Button {
  label: Label,
  callback: Box<dyn FnMut()>,
}

impl Button {
  fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
    Button {
      label: Label::new(label),
      callback,
    }
  }
}

// impl Widget for Button {
//   fn width(&self) -> usize {
//       unimplemented!()
//   }

//   fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
//       unimplemented!()
//   }
// }

pub struct Window {
  title: String,
  widgets: Vec<Box<dyn Widget>>,
}

impl Window {
  fn new(title: &str) -> Window {
    Window {
      title: title.to_owned(),
      widgets: Vec::new(),
    }
  }

  fn add_widget(&mut self, widget: Box<dyn Widget>) {
    self.widgets.push(widget);
  }

  fn inner_width(&self) -> usize {
    std::cmp::max(
      self.title.chars().count(),
      self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
    )
  }
}

// impl Widget for Window {
//   fn width(&self) -> usize {
//       unimplemented!()
//   }

// fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
//       unimplemented!()
//   }
// }

fn simple_gui_library() {
  let mut window = Window::new("Rust GUI Demo 1.23");
  window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
  // window.add_widget(Box::new(Button::new(
  //   "Click me!",
  //   Box::new(|| println!("You clicked the button!")),
  // )));
  // window.draw();
}
