use rand::{rngs::OsRng, RngCore};
use tiny_bip39::{Mnemonic, Language};
use std::io::{self, Write};

/// Generates a BIP-39 compliant seed phrase with the specified number of words.
/// Valid word counts: 12, 15, 18, 21, or 24.
fn generate_seed_phrase(word_count: usize) -> Result<String, String> {
    let entropy_bytes = match word_count {
        12 => 16, // 128 bits
        15 => 20, // 160 bits
        18 => 24, // 192 bits
        21 => 28, // 224 bits
        24 => 32, // 256 bits
        _ => return Err(format!("Invalid word count: {}. Choose from 12, 15, 18, 21, or 24.", word_count)),
    };

    // Generate secure random entropy
    let mut entropy = vec![0u8; entropy_bytes];
    OsRng.fill_bytes(&mut entropy);

    // Generate the mnemonic phrase
    let mnemonic = Mnemonic::from_entropy(&entropy, Language::English)
        .map_err(|e| format!("Error generating mnemonic: {}", e))?;

    Ok(mnemonic.phrase().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Seed Phrase Generator (BIP-39)");
    println!("Enter number of words (12, 15, 18, 21, 24):");
    
    // Flush stdout to ensure prompt is displayed before reading input
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Proper error handling for user input
    let word_count: usize = match input.trim().parse() {
        Ok(count) => count,
        Err(_) => {
            eprintln!("❌ Invalid input! Defaulting to 12 words.");
            12
        }
    };

    match generate_seed_phrase(word_count) {
        Ok(seed_phrase) => {
            println!("\n✅ Your Secure Seed Phrase:");
            println!("🌱 {}", seed_phrase);
            
            // Security warning
            println!("\n⚠️  WARNING: Never share your seed phrase with anyone!");
            println!("⚠️  Store it securely offline in a safe place.");
        }
        Err(err) => eprintln!("❌ Error: {}", err),
    }

    Ok(())
}