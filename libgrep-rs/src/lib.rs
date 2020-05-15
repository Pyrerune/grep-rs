
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
            let mut found: bool = false;
            let list: Vec<&str> = self.search_text.split("\n").collect();
            
            let mut return_string = String::new();
            
/*if self.options.exclude == true {
                for i in list.clone() {
                    if !i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push_str("\n");
                    }
                }
            } else if self.options.exclude == false {
                for i in list.clone() {
                    if i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push_str("\n");
                    }
                }
            }*/
            if self.options.include_before == true {
                for i in list.clone() {
                    if i.contains(self.pattern.as_str()) {
                        if self.options.exclude == false {
                            return_string.push_str(i);
                            return_string.push_str("\n");
                        }
                        found = true;
                    }
                    if found == false {
                        return_string.push_str(i);
                        return_string.push_str("\n");
                    }
                }
            } else if self.options.include_after == true {
                for i in list.clone() {

                    if i.contains(self.pattern.as_str()) {
                        if self.options.exclude == false {
                            return_string.push_str(i);
                            return_string.push_str("\n");
                        }
                        found = true;
                    } else if found == true {
                        return_string.push_str(i);
                        return_string.push_str("\n");
                    }

                }
            } else {
            for i in list {
                if self.options.exclude == true {
                    if !i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push_str("\n")
                    }
                } else if self.options.exclude == false {
                    if i.contains(self.pattern.as_str()) {
                        return_string.push_str(i);
                        return_string.push_str("\n");
                    }
                }
            }
            }
            return_string
        }
    }
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn exclude() {
        let options = Options::new(true, false, false);
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
        let options = Options::new(false, false, false);
        let text = String::from("Steve Jobs Passed Away\nGates thrilled");
        let searcher = Searcher::new(String::from("Jobs"), text, options);
        let assert_text: String = String::from("Steve Jobs Passed Away");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text.remove(return_text.find("\n").unwrap());
        }
        assert_eq!(assert_text, return_text);
    
    }
    #[test]
    fn include_before() {
        let options = Options::new(false, true, false);
        let text = String::from("Steve Jobs Passed Away\nGates thrilled\nApple Fans Devastated\nGates Thrilled and Devastated");
        let searcher = Searcher::new(String::from("Gates"), text, options);
        let assert_text: String = String::from("Steve Jobs Passed Away Gates thrilled Gates Thrilled and Devastated");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text = return_text.replace("\n", " ");
            if return_text.ends_with(" ") {
                return_text.remove(return_text.len() - 1);
            }
            if return_text.starts_with(" ") {
                return_text.remove(0);
            }
            println!("test: {}", return_text);
        }
        assert_eq!(assert_text, return_text);
    }
    #[test]
    fn include_after() {
        let options = Options::new(false, false, true);
        let text = String::from("Steve Jobs Passed Away\nGates thrilled\nApple Fans Devastated");
        let searcher = Searcher::new(String::from("Gates"), text, options);
        let assert_text: String = String::from("Gates thrilled Apple Fans Devastated");
        let mut return_text: String = searcher.search();
        if return_text.contains("\n") {
            return_text = return_text.replace("\n", " ");
            if return_text.ends_with(" ") {
                return_text.remove(return_text.len() - 1);
            }
            if return_text.starts_with(" ") {
                return_text.remove(0);
            }
            println!("test: {}", return_text);
        }
        assert_eq!(assert_text, return_text);
    }
}
