mod format;
mod macros;

use std::error::Error;
use std::fmt::{self, Arguments, Display, Write};

pub use format::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait TreeDisplay {
  fn fmt(&self, tf: &mut TreeFormatter<'_>) -> fmt::Result;
}

pub struct TreeFormatter<'a> {
  inner: &'a mut (dyn Write + 'a),

  level: usize,
  current_context: String,
  context_indices: Vec<usize>,
  context_format: ContextFormat<'a>,
  prefix_format: ItemPrefixFormat<'a>,
}

impl Write for TreeFormatter<'_> {
  #[inline]
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.inner.write_str(s)
  }
}

impl<'a> TreeFormatter<'a> {
  const DEFAULT_BRANCH_CAPACITY: usize = 3;

  pub fn new(title: impl Display, inner: &'a mut impl Write) -> Result<Self> {
    Self::with_context(title, inner, "")
  }

  pub fn with_context(title: impl Display, inner: &'a mut impl Write, context: &'a str) -> Result<Self> {
    writeln!(inner, "{context}{title}")?;
    Ok(Self {
      inner,
      level: 0,
      current_context: String::from(context),
      context_indices: Vec::with_capacity(Self::DEFAULT_BRANCH_CAPACITY),
      context_format: ContextFormat::default(),
      prefix_format: ItemPrefixFormat::default(),
    })
  }

  pub fn context_format(&mut self, context_format: ContextFormat<'a>) -> &mut Self {
    self.context_format = context_format;
    self
  }

  pub fn prefix_format(&mut self, prefix_format: ItemPrefixFormat<'a>) -> &mut Self {
    self.prefix_format = prefix_format;
    self
  }

  pub fn begin_level(&mut self, is_last: bool) {
    self.level += 1;
    if self.level > 1 {
      self.context_indices.push(self.current_context.len());
      self.current_context.push_str(self.context_format.context(is_last));
    }
  }

  pub fn end_level(&mut self) {
    self.level -= 1;
    if let Some(last_idx) = self.context_indices.pop() {
      self.current_context.truncate(last_idx);
    }
  }

  pub fn write(&mut self, is_last: bool, item: impl Display) -> fmt::Result {
    writeln!(self.inner, "{}{}{}", self.current_context, self.prefix_format.prefix(is_last), item)?;
    if is_last && self.context_indices.len() > self.level - 1 {
      let last_idx = self.context_indices.len() - 1;
      self
        .current_context
        .replace_range(self.context_indices[last_idx].., self.context_format.empty);
    }
    Ok(())
  }

  pub fn write_fmt(&mut self, is_last: bool, item: Arguments<'a>) -> fmt::Result {
    self.write(is_last, item)
  }

  pub fn write_level(&mut self, is_last: bool, items: impl Iterator<Item = impl Display>) -> fmt::Result {
    self.begin_level(is_last);
    let mut items = items.peekable();
    while let Some(item) = items.next() {
      self.write(items.peek().is_none(), item)?;
    }
    self.end_level();
    Ok(())
  }
  
  pub fn write_tree(&mut self, tree: impl TreeDisplay) -> fmt::Result {
    tree.fmt(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  fn generate_tree(fmt: &mut TreeFormatter) -> Result<()> {
    tree_indent!(fmt);
    tree_write!(fmt, "Child Level 1 #1")?;
    fmt.write_level(false, (1..=3u8).map(|x| format!("Child Level 2 #{x}")))?;
    tree_write!(fmt, "Child Level 1 #2")?;
    tree_indent!(fmt);
    tree_write!(fmt, "Child Level 2 #4")?;
    tree_write_last!(fmt, "Child Level 2 #5")?;
    tree_indent_last!(fmt);
    tree_write!(fmt, "Child Level 3 #1")?;
    tree_indent!(fmt);
    tree_write!(fmt, "Child Level 4 #1")?;
    tree_write_last!(fmt, "Child Level 4 #2")?;
    tree_unindent!(fmt);
    tree_write_last!(fmt, "Child Level 3 #2")?;
    tree_unindent!(fmt);
    tree_unindent!(fmt);
    tree_write_last!(fmt, "Child Level 1 #3")?;
    tree_indent_last!(fmt);
    tree_write!(fmt, "Child Level 2 #6")?;
    tree_write_last!(fmt, "Child Level 2 #7")?;
    fmt.write_level(true, (3..=5u8).map(|x| format!("Child Level 3 #{x}")))?;
    tree_unindent!(fmt);
    Ok(())
  }

  #[test]
  fn test_tree_formatter() {
    let mut buf = String::new();
    let mut formatter = TreeFormatter::new("Root", &mut buf).unwrap();

    generate_tree(&mut formatter).unwrap();
    assert_eq!(
      buf,
      indoc! {"
        Root
        ├── Child Level 1 #1
        │   ├── Child Level 2 #1
        │   ├── Child Level 2 #2
        │   └── Child Level 2 #3
        ├── Child Level 1 #2
        │   ├── Child Level 2 #4
        │   └── Child Level 2 #5
        │       ├── Child Level 3 #1
        │       │   ├── Child Level 4 #1
        │       │   └── Child Level 4 #2
        │       └── Child Level 3 #2
        └── Child Level 1 #3
            ├── Child Level 2 #6
            └── Child Level 2 #7
                ├── Child Level 3 #3
                ├── Child Level 3 #4
                └── Child Level 3 #5
    "}
    );
  }

  #[test]
  fn test_tree_formatter_custom_context_format() {
    let mut buf = String::new();
    let mut formatter = TreeFormatter::new("Root", &mut buf).unwrap();

    formatter.context_format(ContextFormat::new("* * ", "║ | "));

    generate_tree(&mut formatter).unwrap();
    assert_eq!(
      buf,
      indoc! {"
        Root
        ├── Child Level 1 #1
        ║ | ├── Child Level 2 #1
        ║ | ├── Child Level 2 #2
        ║ | └── Child Level 2 #3
        ├── Child Level 1 #2
        ║ | ├── Child Level 2 #4
        ║ | └── Child Level 2 #5
        ║ | * * ├── Child Level 3 #1
        ║ | * * ║ | ├── Child Level 4 #1
        ║ | * * ║ | └── Child Level 4 #2
        ║ | * * └── Child Level 3 #2
        └── Child Level 1 #3
        * * ├── Child Level 2 #6
        * * └── Child Level 2 #7
        * * * * ├── Child Level 3 #3
        * * * * ├── Child Level 3 #4
        * * * * └── Child Level 3 #5
    "}
    )
  }

  #[test]
  fn test_tree_formatter_custom_prefix_format() {
    let mut buf = String::new();
    let mut formatter = TreeFormatter::new("Root", &mut buf).unwrap();
    formatter.context_format(ContextFormat::new("* * ", "║ | "));
    formatter.prefix_format(ItemPrefixFormat::new("╠═══", "╚═══"));

    generate_tree(&mut formatter).unwrap();
    assert_eq!(
      buf,
      indoc! {"
        Root
        ╠═══Child Level 1 #1
        ║ | ╠═══Child Level 2 #1
        ║ | ╠═══Child Level 2 #2
        ║ | ╚═══Child Level 2 #3
        ╠═══Child Level 1 #2
        ║ | ╠═══Child Level 2 #4
        ║ | ╚═══Child Level 2 #5
        ║ | * * ╠═══Child Level 3 #1
        ║ | * * ║ | ╠═══Child Level 4 #1
        ║ | * * ║ | ╚═══Child Level 4 #2
        ║ | * * ╚═══Child Level 3 #2
        ╚═══Child Level 1 #3
        * * ╠═══Child Level 2 #6
        * * ╚═══Child Level 2 #7
        * * * * ╠═══Child Level 3 #3
        * * * * ╠═══Child Level 3 #4
        * * * * ╚═══Child Level 3 #5
      "}
    )
  }

  #[test]
  fn test_tree_formatter_with_context() {
    let mut buf = String::new();
    let mut formatter = TreeFormatter::with_context("Root", &mut buf, "Some context: ").unwrap();

    generate_tree(&mut formatter).unwrap();
    assert_eq!(
      buf,
      indoc! {"
        Some context: Root
        Some context: ├── Child Level 1 #1
        Some context: │   ├── Child Level 2 #1
        Some context: │   ├── Child Level 2 #2
        Some context: │   └── Child Level 2 #3
        Some context: ├── Child Level 1 #2
        Some context: │   ├── Child Level 2 #4
        Some context: │   └── Child Level 2 #5
        Some context: │       ├── Child Level 3 #1
        Some context: │       │   ├── Child Level 4 #1
        Some context: │       │   └── Child Level 4 #2
        Some context: │       └── Child Level 3 #2
        Some context: └── Child Level 1 #3
        Some context:     ├── Child Level 2 #6
        Some context:     └── Child Level 2 #7
        Some context:         ├── Child Level 3 #3
        Some context:         ├── Child Level 3 #4
        Some context:         └── Child Level 3 #5
     "}
    )
  }
}
