pub use rgrep::*;
mod rgrep {
    #[derive(Clone)]
    pub struct Options {
        pub exclude: bool
    }
    impl Default for Options {
    
        fn default() -> Options {
            Options {
                exclude: false
            }
        }
    }
    impl Options {
        pub fn new(e: bool) -> Options {
            Options {
                exclude: e,
            }
        }
    }
    #[derive(Clone)]
    pub struct Searcher {
        pub pattern: String,
        pub search_text: String,
        pub options: Options,
    }
    impl Searcher {
        pub fn new(p: String, t: String, o: Options) -> Searcher {
            Searcher {
                pattern: p,
                search_text: t,
                options: o
            }
        }
        pub fn search(&self) -> String {
            let list: Vec<&str> = self.search_text.split("\n").collect();
            let mut return_string = String::new();
            if self.options.exclude == true {
                for i in list {
                    if !i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push('\n');
                    }
                }
            } else if self.options.exclude == false {
                for i in list {
                    if i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push('\n');
                    }
                }
            }
            return_string
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::rgrep::*;
    #[test]
    fn exclude() {
        let options = Options::new(true);
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options);
        let assert_text: String = String::from("Gates thrilled");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn include() {
        let options = Options::new(false);
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options);
        let assert_text: String = String::from("Steve Jobs Passed Away");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    
    }
}
