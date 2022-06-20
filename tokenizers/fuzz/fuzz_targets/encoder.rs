#![no_main]
use libfuzzer_sys::fuzz_target;
use tokenizers::tokenizer::{Result, Tokenizer};

fuzz_target!(|data: (&str, &str, bool)| {
    let _ = fuzz(data.0, data.1, data.2);
});

fn fuzz(tokenizer: &str, encode: &str, special: bool) -> Result<()> {
    let tokenizer: Tokenizer = tokenizer.parse()?;
    let _encoding = tokenizer.encode(encode, special)?;
    Ok(())
}
