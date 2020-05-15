pub mod searcher;
pub mod options;
#[cfg(test)]
mod tests {
    use crate::searcher::Searcher;
    use crate::options::Options;
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
