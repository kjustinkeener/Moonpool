# verify-release

Pre-publish check for a Moonpool release: does the signed `moonpool.exe` actually
verify against the public key the shipped app carries?

This exists because the release signing chain has three places to go wrong that a
green CI run does NOT catch:

1. the PowerShell `tauri signer sign` call (an empty `--password ""` gets dropped by
   legacy argument passing, which silently broke v0.1.9 twice),
2. the base64 -> minisign-text decode that produces `update.json`'s `signature`, and
3. any drift between the key CI signs with and `PUBKEY_B64` in `src-tauri/src/update.rs`.

CI happily uploads a release whose signature the updater will reject. Users only find
out when the self-update fails. So verify locally, before publishing the draft.

`src/main.rs` is a deliberate copy of `update.rs::verify_signature`: same public key,
same `minisign-verify` version, `allow_legacy = true`. If that function changes, change
this one too, or the check proves nothing.

## Run it

Git Bash, from the repo root:

```bash
cd /c/claude-local/MoonPool/tools/verify-release && gh release download vX.Y.Z -p moonpool.exe -p update.json -D ./rel && cargo run --release -- ./rel
```

Expected output:

```
VERIFY_OK
tampered rejected: true
```

Both lines matter. The second confirms the verifier actually rejects a modified
payload, so `VERIFY_OK` means something. Anything else: do not publish the draft.

Then publish (Git Bash):

```bash
gh release edit vX.Y.Z --draft=false --latest
```

## Status

Used to gate v0.3.0 (2026-09-03), the first release cut on the custom installer +
updater path. The whole chain came out correct on the first try.
