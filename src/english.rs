use crate::IndexedWord;

pub struct English<'a> {
    text: &'a str,
}

impl<'a> English<'a> {
    pub fn new(text: &'a str) -> English<'a> {
        English { text }
    }
}

impl<'a> Iterator for English<'a> {
    type Item = IndexedWord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy() {
        let mut tokenizer = English::new("hello world!");

        let indexed_word = IndexedWord::new(0, 0, "hello");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(6, 1, "word");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn ponctuation() {
        let mut tokenizer = English::new("hello world. I am happy!");

        let indexed_word = IndexedWord::new(0, 0, "hello");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(6, 1, "word");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(14, 1+8, "I");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(16, 1+8+1, "am");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(19, 1+8+1+1, "happy");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn shortening() {
        let mut tokenizer = English::new("I'm happy!");

        let indexed_word = IndexedWord::new(0, 0, "I");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(2, 1, "m");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        let indexed_word = IndexedWord::new(4, 1+1, "happy");
        assert_eq!(tokenizer.next(), Some(indexed_word));

        assert_eq!(tokenizer.next(), None);
    }
}
