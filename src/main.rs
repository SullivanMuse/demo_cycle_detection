use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
};

/// An example of a recursive data structure
#[derive(Clone)]
enum Value {
    Int(i64),
    Rc(Rc<RefCell<Option<Self>>>),
    List(Vec<Self>),
}

/// Helper function for creating an int value
fn int(x: i64) -> Value {
    Value::Int(x)
}

/// Helper function for creating an rc value
fn rc() -> Value {
    Value::Rc(Rc::new(RefCell::new(None)))
}

/// Helper function for creating a list value
fn list(xs: Vec<Value>) -> Value {
    Value::List(xs)
}

impl Value {
    /// Mutate a refcell to add cycles
    fn resolve(&mut self, value: Self) {
        if let Self::Rc(x) = self {
            *x.borrow_mut() = Some(value);
        } else {
            panic!("Attempt to resolve non-rc value");
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create an instance of the helper data structure
        let mut fmt1 = ValueFormatter::default();

        // Populate the data structure with string snippets
        fmt1.visit(self);

        // Get the resulting string
        let x = fmt1.string();

        // Write the string to the formatter
        write!(fmt, "{}", x)
    }
}

/// Helper data structure
#[derive(Debug, Default)]
struct ValueFormatter {
    visited: HashSet<*const Value>,
    chunks: Vec<String>,
}

impl ValueFormatter {
    /// Add a string to the buffer
    fn add<T: std::fmt::Display>(&mut self, x: T) {
        let x = format!("{}", x);
        self.chunks.push(x);
    }

    /// Get the resulting string
    fn string(&self) -> String {
        self.chunks.join("")
    }

    /// Main recursive function
    fn visit(&mut self, x: &Value) {
        let ptr = x as *const Value;

        // Check if the node has already been visited
        match self.visited.contains(&ptr) {
            // Self-references are represented by "*"
            true => {self.add("*");}

            // Non self-reference
            false => {
                // Insert the node into visited
                self.visited.insert(ptr);
                match x {
                    // Format an Int value
                    Value::Int(x) => {self.add(x);}

                    // Format an Rc value, branching on whether it is initialized or not
                    Value::Rc(x) => match &*x.borrow() {
                        None => self.add("uninit"),
                        Some(x) => self.visit(x),
                    }

                    // Format a List value
                    Value::List(xs) => {
                        self.add("[");
                        if let Some(x) = xs.first() {
                            self.visit(x);
                            for x in &xs[1..] {
                                self.add(", ");
                                self.visit(x);
                            }
                        }
                        self.add("]");
                    }
                }
            }
        }
    }
}

fn main() {
    // Create an uninitialized reference
    let mut x = rc();

    // Create a structure with embedded references
    let y = list(vec![
        int(1),
        int(2),
        int(3),
        x.clone(),
        x.clone(),
        x.clone(),
    ]);

    // uninit
    dbg!(&x);

    // Resolve the reference, creating a cyclic data structure
    x.resolve(y);

    // [1, 2, 3, *, *, *]
    dbg!(&x);
}
