pub struct ContextFormat<'a> {
  pub(super) empty: &'a str,
  pub(super) level: &'a str,
}

impl<'a> ContextFormat<'a> {
  const LEVEL_CONTEXT: &'static str = "│   ";
  const EMPTY_CONTEXT: &'static str = "    ";

  pub fn new(empty_context: &'a str, level_context: &'a str) -> Self {
    ContextFormat { empty: empty_context, level: level_context }
  }

  #[inline(always)]
  pub(super) fn context(&self, is_last: bool) -> &'a str {
    match is_last {
      true => self.empty,
      false => self.level,
    }
  }

  pub const fn default() -> Self {
    Self { empty: Self::EMPTY_CONTEXT, level: Self::LEVEL_CONTEXT }
  }
}

pub struct ItemPrefixFormat<'a> {
  item: &'a str,
  last: &'a str,
}

impl<'a> ItemPrefixFormat<'a> {
  const DEFAULT_ITEM_PREFIX: &'static str = "├── ";
  const DEFAULT_LAST_ITEM_PREFIX: &'static str = "└── ";

  pub fn new(item_prefix: &'a str, last_item_prefix: &'a str) -> Self {
    Self { item: item_prefix, last: last_item_prefix }
  }

  #[inline(always)]
  pub(super) fn prefix(&self, is_last: bool) -> &'_ str {
    match is_last {
      true => self.last,
      false => self.item,
    }
  }

  pub const fn default() -> Self {
    Self {
      item: Self::DEFAULT_ITEM_PREFIX,
      last: Self::DEFAULT_LAST_ITEM_PREFIX,
    }
  }
}
