
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Unordered pair of Computers.
/// 
/// https://stackoverflow.com/a/71162181
pub struct Connection {
    pub a: String,
    pub b: String
}

impl Connection {

    /// Create new Connection.
    /// 
    /// Enforces that a <= b when creating
    pub fn new(a: String, b: String) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn in_conn(&self, comp: &String) -> bool {
        self.a == *comp || self.b == *comp
    }

    /// Returns the other computer in the connection
    /// or `None` if comp is not in the conneciton
    pub fn other(&self, comp: &String) -> Option<String> {
        if !self.in_conn(comp) { None } 
        else {
            if *comp == self.a {
                Some(self.b.clone())
            } else {
                Some(self.a.clone())
            }
        }
    }
}

