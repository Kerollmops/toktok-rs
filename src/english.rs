use crate::IndexedWord;

pub struct Text<'a> {
    content: &'a str,
    char_index: usize,
    word_index: usize,
    byte_index: usize, //maybe not necessary with: `pub fn char_indices(&self) -> CharIndices`
}

impl<'a> Text<'a> {
    fn new(content: &'a str) -> Text<'a> {
        Text { content, char_index: 0, word_index: 0, byte_index: 0 }
    }
}

pub struct English<'a> {
    text: Text<'a>,
}

impl<'a> English<'a> {
    pub fn new(text: &'a str) -> English<'a> {
        English { text: Text::new(text) }
    }
}

impl<'a> Iterator for English<'a> {
    type Item = IndexedWord<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        // TODO: Calculate Everything here to find above values
        let token_len = 5; // Size of the word to calculate
        let token_byte_size = 5; // Size of the word in byte to calculate
        let token_weight = 1; // 8 if delimitation like ('.', ',', ';', '...')

        let indexed_word = IndexedWord::new(
            self.text.char_index,
            self.text.byte_index,
            self.text.word_index,
            &self.text.content[..token_len]
        );

        // DANGER: token_len doesn't count delimitations.
        self.text.content = &self.text.content[token_len..];
        self.text.char_index += token_len; 
        self.text.byte_index += token_byte_size;
        self.text.word_index += token_weight;

        Some(indexed_word)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy() {
        let mut tokenizer = English::new("hello world!");

        let indexed_word = IndexedWord::new(0, 0, 0, "hello");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(6, 6, 1, "word");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn ponctuation() {
        let mut tokenizer = English::new("hello world. I am happy!");

        let indexed_word = IndexedWord::new(0, 0, 0, "hello");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(6, 6, 1, "world");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(14, 14, 1+8, "I");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(16, 16, 1+8+1, "am");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(19, 19, 1+8+1+1, "happy");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn shortening() {
        let mut tokenizer = English::new("I'm happy!");

        let indexed_word = IndexedWord::new(0, 0, 0, "I");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(2, 2, 1, "m");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(4, 4, 1+1, "happy");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }
}
