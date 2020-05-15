pub use options::*;
mod options {
    #[derive(Clone)]
    pub struct Options {
        pub exclude: bool,
        pub include_before: bool,
        pub include_after: bool,
    }
    impl Default for Options {
    
        fn default() -> Options {
            Options {
                exclude: false,
                include_before: false,
                include_after: false,
            }
        }
    }
    impl Options {
        pub fn new(e: bool, ib: bool, ia: bool) -> Options {
            Options {
                exclude: e,
                include_before: ib,
                include_after: ia,
            }
        }
    }
}