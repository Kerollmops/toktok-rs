mod english;

pub use english::English;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedWord<'a> {
    pub char_index: usize,
    pub byte_index: usize,
    pub word_index: usize,
    pub word: &'a str,
}

impl<'a> IndexedWord<'a> {
    fn new(char_index: usize, byte_index: usize, word_index: usize, word: &str) -> IndexedWord {
        IndexedWord { char_index, byte_index, word_index, word }
    }
}
