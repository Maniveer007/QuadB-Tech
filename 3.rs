fn find_shortest_word(sentence: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    let mut shortest_length = usize::MAX;

    for word in sentence.split_whitespace() {
        let word_length = word.chars().count();
        if word_length < shortest_length {
            shortest_word = Some(word);
            shortest_length = word_length;
        }
    }

    shortest_word
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    match find_shortest_word(sentence) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found"),
    }
}
