#[macro_export]
macro_rules! tree_write {
  ($dst:expr, $($args:tt)*) => {
    $dst.write_fmt(false, format_args!($($args)*))
  };
}

#[macro_export]
macro_rules! tree_write_last {
  ($dst:expr, $($args:tt)*) => {
    $dst.write_fmt(true, format_args!($($args)*))
  }
}

#[macro_export]
macro_rules! tree_indent {
  ($dst:expr) => {
    $dst.begin_level(false);
  }
}

#[macro_export]
macro_rules! tree_indent_last {
  ($dst:expr) => {
    $dst.begin_level(true);
  }
}

#[macro_export]
macro_rules! tree_unindent {
  ($dst:expr) => {
    $dst.end_level();
  }
}
