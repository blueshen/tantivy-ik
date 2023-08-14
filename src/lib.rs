use ik_rs::core::ik_segmenter::{IKSegmenter, TokenMode};
use once_cell::sync::Lazy;

cfg_if::cfg_if! {
    if #[cfg(feature="use-parking-lot")] {
        use parking_lot::RwLock;
    }
    else /*if #[cfg(feature="use-std-sync")]*/ {
        use std::sync::RwLock;
    }
}

use tantivy::tokenizer::{Token, TokenStream, Tokenizer};

pub static GLOBAL_IK: Lazy<RwLock<IKSegmenter>> = Lazy::new(|| {
    let ik = IKSegmenter::new();
    RwLock::new(ik)
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
    type TokenStream<'a> = IkTokenStream;
    fn token_stream<'a>(&mut self, text: &'a str) -> Self::TokenStream<'a> {
        let mut indices = text.char_indices().collect::<Vec<_>>();
        indices.push((text.len(), '\0'));

        let lock_guard = {cfg_if::cfg_if! {
            if #[cfg(feature="use-parking-lot")] {Some(GLOBAL_IK.read())}
            else /*if #[cfg(feature="use-std-sync")]*/ {
                match GLOBAL_IK.read() {
                    Err(_err) => None,
                    Ok(lck) => Some(lck)
                }
            }
        }};
        let orig_tokens = lock_guard.map_or(vec![],|seg|seg.tokenize(text, self.mode.clone()));

        let mut tokens = Vec::new();
        for token in orig_tokens.iter() {
            tokens.push(Token {
                offset_from: indices[token.begin_pos()].0,
                offset_to: indices[token.end_pos()].0,
                position: token.begin_pos(),
                text: String::from(
                    &text[(indices[token.begin_pos()].0)..(indices[token.end_pos()].0)],
                ),
                position_length: token.len(),
            });
        }
        IkTokenStream { tokens, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenMode;

    #[test]
    fn tantivy_ik_works() {
        use tantivy::tokenizer::*;
        let mut tokenizer = crate::IkTokenizer::new(TokenMode::SEARCH);
        let mut token_stream = tokenizer.token_stream(
            "张华考上了北京大学；李萍进了中等技术学校；我在百货公司当售货员：我们都有光明的前途",
        );
        let mut tokens = Vec::new();
        let mut token_text = Vec::new();
        while let Some(token) = token_stream.next() {
            tokens.push(token.clone());
            token_text.push(token.text.clone());
        }
        // offset should be byte-indexed
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, "张华".bytes().len());
        assert_eq!(tokens[1].offset_from, "张华".bytes().len());
        // check tokenized text
        assert_eq!(
            token_text,
            vec![
                "张华",
                "考",
                "上了",
                "北京大学",
                "李萍",
                "进了",
                "中等",
                "技术学校",
                "我",
                "在",
                "百货公司",
                "当",
                "售货员",
                "我们",
                "都有",
                "光明",
                "的",
                "前途"
            ]
        );
    }
}
