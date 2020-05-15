pub use searcher::*;
mod searcher {
    use crate::options::Options;
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

}