use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tantivy::tokenizer::{BoxTokenStream, Token, TokenStream, Tokenizer};

pub static GLOBAL_IK: Lazy<Mutex<IKSegmenter>> = Lazy::new(|| {
    let ik = IKSegmenter::new();
    Mutex::new(ik)
});

#[derive(Clone)]
pub struct IkTokenizer {
    mode: TokenMode,
}

pub struct IkTokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream for IkTokenStream {
    fn advance(&mut self) -> bool {
        if self.index < self.tokens.len() {
            self.index = self.index + 1;
            true
        } else {
            false
        }
    }
    fn token(&self) -> &Token {
        &self.tokens[self.index - 1]
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.tokens[self.index - 1]
    }
}

impl IkTokenizer {
    pub fn new(mode: TokenMode) -> Self {
        Self { mode }
    }
}

impl Tokenizer for IkTokenizer {
    fn token_stream<'a>(&self, text: &'a str) -> BoxTokenStream<'a> {
        let mut indices = text.char_indices().collect::<Vec<_>>();
        indices.push((text.len(), '\0'));
        let orig_tokens = GLOBAL_IK.lock().unwrap().tokenize(text, self.mode);
        let mut tokens = Vec::new();
        for token in orig_tokens.iter() {
            tokens.push(Token {
                offset_from: indices[token.get_begin_position()].0,
                offset_to: indices[token.get_end_position()].0,
                position: token.get_begin_position(),
                text: String::from(
                    &text[(indices[token.get_begin_position()].0)
                        ..(indices[token.get_end_position()].0)],
                ),
                position_length: token.get_length(),
            });
        }
        BoxTokenStream::from(IkTokenStream { tokens, index: 0 })
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenMode;

    #[test]
    fn tantivy_ik_works() {
        use tantivy::tokenizer::*;
        let tokenizer = crate::IkTokenizer::new(TokenMode::SEARCH);
        let mut token_stream = tokenizer.token_stream(
            "???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????",
        );
        let mut tokens = Vec::new();
        let mut token_text = Vec::new();
        while let Some(token) = token_stream.next() {
            tokens.push(token.clone());
            token_text.push(token.text.clone());
        }
        // offset should be byte-indexed
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, "??????".bytes().len());
        assert_eq!(tokens[1].offset_from, "??????".bytes().len());
        // check tokenized text
        assert_eq!(
            token_text,
            vec![
                "??????",
                "???",
                "??????",
                "????????????",
                "??????",
                "??????",
                "??????",
                "????????????",
                "???",
                "???",
                "????????????",
                "???",
                "?????????",
                "??????",
                "??????",
                "??????",
                "???",
                "??????"
            ]
        );
    }
}
