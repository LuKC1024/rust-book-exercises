//! P2: Terminal user interface
//!
//! This problem explores the differences between designing systems with
//! classes and traits. The adjacent file `tui.cpp` provides a C++ implementation
//! of a terminal user inteface (TUI), i.e. a simple set of graphical elements that
//! can be drawn into the terminal. The C++ code uses classes, inheritance, and
//! virtual methods.
//!
//! To see the C++ code in action, you can build and execute the code by running:
//!
//! ```bash
//! ./run.cpp-sh
//! ```
//!
//! Your task is to redesign the C++ TUI API into Rust. Your API should similarly
//! contain data structures that represent Text, Heading, and Container. You should
//! replicate the behavior of `main` in `tui.cpp` into the `container_test` function.

struct Dimensions {
    width: usize,
    height: usize,
}

trait Element {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: 0,
            height: 0,
        }
    }
    fn render(&self) {}
}

struct Text {
    text: String,
}
impl Text {
    fn new(text: String) -> Self {
        Text { text }
    }
}
impl Element for Text {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.text.len(),
            height: 0,
        }
    }
    fn render(&self) {
        print!("{}", self.text)
    }
}

struct Heading {
    text: Text,
}
impl Heading {
    fn new(text: String) -> Self {
        Heading {
            text: Text::new(text),
        }
    }
}
impl Element for Heading {
    fn dimensions(&self) -> Dimensions {
        self.text.dimensions()
    }
    fn render(&self) {
        print!("\u{001b}[1m");
        self.text.render();
        print!("\u{001b}[0m");
    }
}

struct Container {
    children: Vec<Box<dyn Element>>,
}
impl Container {
    fn new(children: Vec<Box<dyn Element>>) -> Self {
        Container { children }
    }
    fn render_line(&self) {
        let dims = self.dimensions();
        print!("+");
        print!("{}", "-".repeat(dims.width - 2));
        print!("+")
    }
}
impl Element for Container {
    fn dimensions(&self) -> Dimensions {
        let ws = self.children.iter().map(|c| c.dimensions().width);
        let hs = self.children.iter().map(|c| c.dimensions().height);
        Dimensions {
            width: ws.max().unwrap_or(0) + 2,
            height: hs.sum(),
        }
    }
    fn render(&self) {
        let dim_s = self.dimensions();
        self.render_line();
        for c in self.children.iter() {
            let dim_c = c.dimensions();
            print!("|");
            c.render();
            print!("{}", " ".repeat(dim_s.width - 2 - dim_c.width));
            println!("|");
        }
        self.render_line();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn container_test() {
        let text = Heading::new(String::from("Hello world!"));
        let text2 = Text::new(String::from("This is a long string of text"));
        let container = Container::new(vec![Box::new(text), Box::new(text2)]);
        container.render()
    }
}
