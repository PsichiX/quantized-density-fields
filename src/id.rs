use std::fmt;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Id(Uuid);

impl Id {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for Id {
    #[inline]
    fn default() -> Self {
        Id(Uuid::new_v4())
    }
}

impl fmt::Debug for Id {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Id {
    #[inline]
    fn to_string(&self) -> String {
        format!("Id({})", self.0)
    }
}
