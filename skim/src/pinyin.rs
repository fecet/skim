//! Pinyin functionality for skim
//! 
//! This module provides utilities to convert Chinese characters to pinyin
//! for fuzzy matching purposes while preserving the original display text.

#[cfg(feature = "pinyin")]
use pinyin::ToPinyin;
use std::borrow::Cow;
use std::sync::Arc;

use crate::{SkimItem, AnsiString, DisplayContext, PreviewContext, ItemPreview};

/// Convert a string containing Chinese characters to pinyin
#[cfg(feature = "pinyin")]
pub fn to_pinyin(text: &str) -> String {
    let mut converted: Vec<String> = vec![];
    let chars: Vec<char> = text.chars().collect();
    for (idx, pinyin_result) in text.to_pinyin().enumerate() {
        if let Some(pinyin_result) = pinyin_result {
            converted.push(pinyin_result.plain().to_string());
        } else {
            // If character is not Chinese, keep the original character
            converted.push(chars[idx].to_string());
        }
    }
    converted.join("")
}

/// Fallback function when pinyin feature is disabled
#[cfg(not(feature = "pinyin"))]
pub fn to_pinyin(text: &str) -> String {
    text.to_string()
}

/// A wrapper around any SkimItem that provides pinyin matching
pub struct PinyinItem {
    inner: Arc<dyn SkimItem>,
    pinyin_text: String,
}

impl PinyinItem {
    /// Create a new PinyinItem that wraps an existing SkimItem
    pub fn new(item: Arc<dyn SkimItem>) -> Self {
        let pinyin_text = to_pinyin(&item.text());
        Self {
            inner: item,
            pinyin_text,
        }
    }
}

impl SkimItem for PinyinItem {
    /// Return the pinyin version of the text for matching
    fn text(&self) -> Cow<'_, str> {
        Cow::from(&self.pinyin_text)
    }

    /// Display the original text (not pinyin) to the user
    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        self.inner.display(context)
    }

    /// Use the original item's preview
    fn preview(&self, context: PreviewContext) -> ItemPreview {
        self.inner.preview(context)
    }

    /// Return the original text for output
    fn output(&self) -> Cow<'_, str> {
        self.inner.output()
    }

    /// Use the original item's matching ranges
    fn get_matching_ranges(&self) -> Option<&[(usize, usize)]> {
        self.inner.get_matching_ranges()
    }

    /// Use the original item's index
    fn get_index(&self) -> usize {
        self.inner.get_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "pinyin")]
    fn test_pinyin_conversion() {
        let result = to_pinyin("你好世界");
        assert_eq!(result, "nihaoshijie");
    }

    #[test]
    #[cfg(feature = "pinyin")]
    fn test_pinyin_mixed_text() {
        let result = to_pinyin("hello你好world");
        assert_eq!(result, "hellonihaoworld");
    }

    #[test]
    #[cfg(not(feature = "pinyin"))]
    fn test_pinyin_fallback() {
        let result = to_pinyin("你好世界");
        assert_eq!(result, "你好世界");
    }
}