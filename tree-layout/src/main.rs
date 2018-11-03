use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeSet;
use std::path::Path;

/// Indent string (prefix) to use for deepening directories.
const INDENT_STRING: &str = "  ";

/// Special string to identify the root.
const ROOT_STRING: &str = "/";

/// Suffix to append to a directory in the tree-layout view.
const DIR_SUFFIX: &str = "/";

// FIXME: the DIR_SUFFIX doesn’t really make sense with that encoding because empty directories are
// encoded as regular files, which is wrong.
/// A node in the file tree.
#[derive(Debug)]
struct FileNode {
  /// Name of the file.
  name: String,

  /// Possible children of the file.
  ///
  /// # Details
  ///
  /// The Rc is required in order to cheap-index a tree with a zipper. The RefCell adds internal
  /// mutability to that sharing.
  children: BTreeSet<Rc<RefCell<FileNode>>>
}

impl PartialEq for FileNode {
  fn eq(&self, rhs: &Self) -> bool {
    self.name.eq(&rhs.name)
  }
}

impl Eq for FileNode {}

impl PartialOrd for FileNode {
  fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
    self.name.partial_cmp(&rhs.name)
  }
}

impl Ord for FileNode {
  fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
    self.name.cmp(&rhs.name)
  }
}

impl FileNode {
  /// Create a new file node based on its name.
  fn new<N>(name: N) -> Self where N: Into<String> {
    FileNode {
      name: name.into(),
      children: BTreeSet::new()
    }
  }

  /// Generate a file tree based on several paths.
  fn from_paths<'a, P>(paths: P) -> Self where P: 'a + IntoIterator<Item = &'a Path> {
    let tree = Rc::new(RefCell::new(FileNode::new(ROOT_STRING)));
    let mut zipper; // cheap zipper (shared counted ref) to iterate through the building index

    for path in paths {
      // reset the zipper to the root in order to start indexing a new path
      zipper = tree.clone();

      // skip the first component, that is the root
      for component in path.components().skip(1) {
        // TODO: this will only work with root-suffixed paths; if we try to pass something else, we
        // break the code
        let node = Rc::new(RefCell::new(FileNode::new((component.as_ref() as &Path).to_str().unwrap())));

        // FIXME: a bit hacky as BTreeSet doesn’t allow for a closure-based lookup; a better
        // representation would be a BTreeMap<PathBuf, FileChildren>.
        let child = zipper.borrow().children.get(&node).cloned(); 

        match child {
          Some(child) => {
            zipper = child; // advance
          }

          None => {
            // unknown file: add it to the index
            zipper.borrow_mut().children.insert(node.clone());
            zipper = node; // advance
          }
        }
      }
    }
    
    // unwrap is safe here as all borrows are dropped
    Rc::try_unwrap(tree).unwrap().into_inner()
  }

  /// Render the file tree.
  fn render(&self, depth: u32) -> String {
    // render the current node first
    let mut rendered = INDENT_STRING.repeat(depth as usize) + &self.name;

    if !self.children.is_empty() && self.name != DIR_SUFFIX {
      rendered += DIR_SUFFIX;
    }

    // then render all the children
    let child_depth = depth + 1;
    for child in self.children.iter() {
      rendered += &format!("\n{}", child.borrow().render(child_depth));
    }

    rendered
  }
}

/// Render a list of paths with a tree layout.
fn render(paths: &[&Path]) -> String {
  // the cloned works on &&, which is just a chead deref
  FileNode::from_paths(paths.iter().cloned()).render(1)
}

fn main() {
  let paths = &[
    Path::new("/foo/bar"),
    Path::new("/foo/bar/example.txt"),
    Path::new("/foo/bar/I_want_to_break_free.ogg"),
    Path::new("/foo/quux/zoo.txt")
  ];

  let _ = FileNode::new("lol");

  println!("{}", render(paths));
}
