//! Example demonstrating pinyin functionality in skim
//! 
//! This example shows how to use the pinyin feature to search Chinese characters
//! using their pinyin pronunciation.

#[cfg(feature = "pinyin")]
use skim::prelude::*;
#[cfg(feature = "pinyin")]
use std::io::Cursor;

#[cfg(feature = "pinyin")]
pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%".to_string()))
        .multi(true)
        .pinyin(true)  // Enable pinyin mode
        .build()
        .unwrap();

    // Create some Chinese text
    let input = "你好世界\nhello\n测试pinyin\nworld\n中文搜索".to_string();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // Transform items to support pinyin
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    
    std::thread::spawn(move || {
        for item in items {
            let pinyin_item: Arc<dyn SkimItem> = Arc::new(PinyinItem::new(item));
            if tx_item.send(pinyin_item).is_err() {
                break;
            }
        }
    });

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        println!("{}", item.output());
    }
}

#[cfg(not(feature = "pinyin"))]
pub fn main() {
    eprintln!("This example requires the 'pinyin' feature to be enabled.");
    eprintln!("Run with: cargo run --features pinyin --example pinyin_demo");
    std::process::exit(1);
}