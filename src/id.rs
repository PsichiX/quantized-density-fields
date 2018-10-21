use std::fmt;
use uuid::Uuid;

/// Universal Identifier (uuidv4).
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct ID(Uuid);

impl ID {
    /// Creates new identifier.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets underlying UUID object.
    #[inline]
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ID {
    #[inline]
    fn default() -> Self {
        ID(Uuid::new_v4())
    }
}

impl fmt::Debug for ID {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for ID {
    #[inline]
    fn to_string(&self) -> String {
        format!("ID({})", self.0)
    }
}
