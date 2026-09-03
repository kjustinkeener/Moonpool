//! Verify a Moonpool release the way the shipped updater will.
//!
//! Mirrors `src-tauri/src/update.rs::verify_signature` byte for byte: same
//! committed public key, same `minisign-verify` crate, same `allow_legacy = true`.
//! Run it against a downloaded release BEFORE publishing the draft - it is the
//! only check that proves the CI signing step and the base64 -> minisig decode
//! actually produced something the app will accept.
//!
//! Usage (Git Bash), from this directory:
//!   gh release download vX.Y.Z -p moonpool.exe -p update.json -D /tmp/rel
//!   cargo run --release -- /tmp/rel
//!
//! Expects `<dir>/moonpool.exe` and `<dir>/update.json`. Prints VERIFY_OK on
//! success, and separately confirms that a tampered payload IS rejected (a
//! verifier that accepts everything would also print OK).
//!
//! If `PUBKEY_B64` below ever drifts from update.rs, this tool is worthless -
//! keep them identical.

const PUBKEY_B64: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJBRkE1M0EzNkUyQzI3MkMKUldRc0p5eHVvMVA2dW1WbUlZSzFPOGY1UjNWQUwvNnFCTmtncmNYeWcvUjZrQllPTER4QUliY00K";
fn main() {
    use base64::Engine as _;
    let dir = std::env::args().nth(1).unwrap();
    let data = std::fs::read(format!("{dir}/moonpool.exe")).unwrap();
    let man: serde_json::Value = serde_json::from_slice(&std::fs::read(format!("{dir}/update.json")).unwrap()).unwrap();
    let sig_text = man["signature"].as_str().unwrap();
    let pubkey_file = String::from_utf8(base64::engine::general_purpose::STANDARD.decode(PUBKEY_B64).unwrap()).unwrap();
    let key_line = pubkey_file.lines().find(|l| !l.trim().is_empty() && !l.starts_with("untrusted comment:")).unwrap();
    let pk = minisign_verify::PublicKey::from_base64(key_line.trim()).unwrap();
    let sig = minisign_verify::Signature::decode(sig_text).expect("parse signature");
    match pk.verify(&data, &sig, true) { Ok(()) => println!("VERIFY_OK"), Err(e) => { println!("VERIFY_FAILED: {e:?}"); std::process::exit(1) } }
    let tampered: Vec<u8> = data.iter().copied().chain([0u8]).collect();
    println!("tampered rejected: {}", pk.verify(&tampered, &sig, true).is_err());
}
