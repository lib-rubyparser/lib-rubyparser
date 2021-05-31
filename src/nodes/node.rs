use crate::Loc;
use crate::Node;

impl Node {
    /// Returs a whitequark/parser -like representation of `self`.
    ///
    /// Used in tests and example scripts
    pub fn inspect(&self, indent: usize) -> String {
        self.inner_ref().inspect(indent)
    }

    /// Returns location of the full node expression
    pub fn expression(&self) -> &Loc {
        self.inner_ref().expression()
    }

    /// Returns a whitequark/parser -like node name.
    ///
    /// Used in tests and example scripts
    pub fn str_type(&self) -> &'static str {
        self.inner_ref().str_type()
    }

    /// Prints itself + location information
    pub fn print_with_locs(&self) {
        self.inner_ref().print_with_locs()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "link-external-c-structures")]
    #[test]
    fn test_size_c() {
        use super::Node;
        assert_eq!(std::mem::size_of::<Node>(), 192);
    }

    #[cfg(feature = "link-external-cpp-structures")]
    #[test]
    fn test_size_cpp() {
        use super::Node;
        assert_eq!(std::mem::size_of::<Node>(), 184);
    }
}
