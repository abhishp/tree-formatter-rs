# tree-formatter-rs

A simple library to format hierarchical structures as text-based trees.  

## Usage

This library provides macros and formatting utilities to easily create and display tree-like structures in text output. You can customize the appearance of the tree branches and item prefixes.

**Example:**

Assuming you have a formatter object (e.g., `tree_formatter`) that implements the necessary methods (`begin_level`, `end_level`, `write_fmt`), you can build a tree like this:

```rust
// Import the macros (adjust path as necessary)
// use tree_formatter::{tree_indent, tree_indent_last, tree_unindent, tree_write, tree_write_last};

// Assuming 'tree_formatter' is initialized, potentially with custom formats:
// let context_fmt = ContextFormat::new("    ", "│   "); // Example: default format
// let item_fmt = ItemPrefixFormat::new("├── ", "└── "); // Example: default format
// let mut tree_formatter = TreeFormatter::new(context_fmt, item_fmt); // Hypothetical writer

tree_write!(tree_formatter, "Root Node");

// Start the first branch
tree_indent!(tree_formatter);
tree_write!(tree_formatter, "Child 1");

// Start a nested branch
tree_indent!(tree_formatter);
tree_write!(tree_formatter, "Grandchild 1.1");
tree_write_last!(tree_formatter, "Grandchild 1.2");
tree_unindent!(tree_formatter); // End nested branch

tree_write_last!(tree_formatter, "Child 2 (last in this branch)");
tree_unindent!(tree_formatter); // End first branch

// Start the second (and last) branch from root
tree_indent_last!(tree_formatter);
tree_write!(tree_formatter, "Child 3");
tree_write_last!(tree_formatter, "Child 4 (last overall)");
tree_unindent!(tree_formatter); // End second branch

// Get the final output from the writer
// let output = tree_formatter.to_string();
// println!("{}", output);
```
