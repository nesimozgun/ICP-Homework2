use std::io; // use io library for input-output process

// create a struct with a ´WordCounter´ name and ´text´ field
struct WordCounter {
    text: String
}

// implement functions for ´WordCounter´ struct
impl WordCounter {

    // create a new instance of the ´WordCounter´ 
    fn new(text: &str) -> Self {
        // create a new ´WordCounter´ object
        WordCounter {
            text: String::from(text) // create a new ´String´ instance from the ´text´ argument
        }
    }

    // split the text input and count its pieces (words)
    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    println!("Type something:"); // prompt the user to enter a text
    let mut input = String::new(); // create an input variable for the input will be given 
    io::stdin().read_line(&mut input).expect("Failed to read line"); // read the input and handle errors
    let counter = WordCounter::new(&input); // create a ´WordCounter´ instance
    let word_count = counter.count_words(); // count words
    println!("Word count: {}", word_count);} // print count