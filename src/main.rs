use std::error::Error;
use gpt3::generate_text;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = r#"This is a diff from a recent change of the ore-rs Rust library. Explain what was changed.

    diff --git a/src/scheme/bit2.rs b/src/scheme/bit2.rs
    index 9dd0b4d..ab244a7 100644
    --- a/src/scheme/bit2.rs
    +++ b/src/scheme/bit2.rs
    @@ -39,11 +39,7 @@ type EncryptLeftResult<R, const N: usize> = Result<Left<OreAes128<R>, N>, OREErr
     type EncryptResult<R, const N: usize> = Result<CipherText<OreAes128<R>, N>, OREError>;
     
     fn cmp(a: u8, b: u8) -> u8 {
    -    if a > b {
    -        1u8
    -    } else {
    -        0u8
    -    }
    +    u8::from(a > b)
     }
     
     impl<R: Rng + SeedableRng> ORECipher for OreAes128<R> {
    "#;
    let api_key = "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";
    let client = reqwest::Client::new();

    let answer = generate_text(&client, api_key, "alpha", prompt, 0.8, 100).await?;
    println!("Answer: {}", answer);

    Ok(())
}
