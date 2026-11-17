# Mainnet deployment

This fork runs upstream's programs under its own program IDs. Nothing else differs from upstream at the base commit.

| Program | Program ID | Library |
| --- | --- | --- |
| Ed25519 Signer | `Sig8PHhDifGhptyvrxGRoNNHJSvKYBfSjbxBchTay9E` | `spl_ed25519_signer_program` |
| Message Executor | `ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp` | `spl_message_executor_program` |
| Nonce | `NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h` | `spl_nonce_program` |

The executor's transition signer (`["transition-signer"]`, bump 255) is `8foN7sp8oHRj51Zq8t2ctrU2SjxYETjsYTnBn2AFVMBF`.

The program keypairs are not in this repository. Keep them offline; after the first deploy, only the upgrade authority matters.

## Build

A verifiable build, so anyone can reproduce the deployed bytes from this commit (Docker, [solana-verify](https://github.com/Ellipsis-Labs/solana-verifiable-build)):

```bash
for lib in spl_nonce_program spl_message_executor_program spl_ed25519_signer_program; do
  solana-verify build --library-name $lib --base-image solanafoundation/solana-verifiable-build:3.1.8
done
```

Built at `8964545` (the program-ID commit; this file came after and changes no code):

| Library | Size | SHA-256 of the `.so` | `solana-verify get-executable-hash` |
| --- | --- | --- | --- |
| `spl_ed25519_signer_program` | 166,600 | `bd541ad39f32464cd924a068de45548f293e0332bc74d60a60c753f89a29c3bd` | `549e443ea960e9933925c2542233ab9b3ee5a8120f72c7d21623295c411c2fd5` |
| `spl_message_executor_program` | 97,976 | `762b446529ac50254230d33dbf68e5cb85502f3c7539040967012c55204185aa` | `b064079f0761d2c4245c4b80c8e4055f13948b4e9b7ac651642d76768e70293c` |
| `spl_nonce_program` | 10,640 | `bdb3d70d542580f81a095322d50bf738f2ec39baf9b887970c4093f00653c4e2` | `70764f1ed24d55ed3076669efd339d1eba4b43117a1d9c1204c9bd39fdb573f2` |

A plain `cargo build-sbf` on another machine gives other bytes for the signer and executor (paths in panic messages), so deploy the `solana-verify` build.

## Deploy

With `KEYS` the folder holding the three program keypairs and `AUTH` the upgrade authority's keypair, which also pays.
The order does not matter: nothing checks the other programs at deploy time.

```bash
for lib in spl_nonce_program spl_message_executor_program spl_ed25519_signer_program; do
  solana program deploy -u mainnet-beta \
    --program-id "$KEYS/$lib-keypair.json" \
    --upgrade-authority "$AUTH" --fee-payer "$AUTH" \
    --with-compute-unit-price 100000 --use-rpc \
    target/deploy/$lib.so
done
```

Rent for the three program data accounts is about 1.9 SOL (the signer about 1.16, the executor about 0.68, the nonce about 0.08); a buffer is about as much again while a deploy runs and comes back when it completes. Leave out `--max-len`: extra space leaves zeros after the program, which changes the hash of the bytes read back. If a deploy stops part way, `solana program show --buffers -u mainnet-beta` lists the buffer to resume with `--buffer` or close.

Then check each against the build:

```bash
solana-verify get-program-hash -u mainnet-beta Sig8PHhDifGhptyvrxGRoNNHJSvKYBfSjbxBchTay9E
solana-verify get-program-hash -u mainnet-beta ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp
solana-verify get-program-hash -u mainnet-beta NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h
```

## IDL

Write each program's IDL to its program-metadata account, signed by the upgrade authority. Write them from LF files (a Windows checkout with `core.autocrlf` has CRLF ones, which hash differently from what Airlock pins): `git show HEAD:idl.json > /tmp/idl.json` gives the committed bytes.

```bash
git show HEAD:idl.json > /tmp/signer-idl.json
git show HEAD:executor/idl.json > /tmp/executor-idl.json
git show HEAD:nonce/idl.json > /tmp/nonce-idl.json
npx @solana-program/program-metadata@0.11.0 -k "$AUTH" --rpc https://api.mainnet-beta.solana.com write idl Sig8PHhDifGhptyvrxGRoNNHJSvKYBfSjbxBchTay9E /tmp/signer-idl.json
npx @solana-program/program-metadata@0.11.0 -k "$AUTH" --rpc https://api.mainnet-beta.solana.com write idl ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp /tmp/executor-idl.json
npx @solana-program/program-metadata@0.11.0 -k "$AUTH" --rpc https://api.mainnet-beta.solana.com write idl NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h /tmp/nonce-idl.json
```

The root `idl.json` (SHA-256 `9f9a17d7b7f1d2d77cfcd5f1a482e671516bc90c48f12e650b7c6a90ea53897a`) is the one Airlock ships: it carries the executor and nonce program as additional programs.

## Verify

Once pushed and deployed, record the build on chain so OtterSec's verifier can reproduce it:

```bash
for p in "Sig8PHhDifGhptyvrxGRoNNHJSvKYBfSjbxBchTay9E spl_ed25519_signer_program" \
         "ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp spl_message_executor_program" \
         "NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h spl_nonce_program"; do
  set -- $p
  solana-verify verify-from-repo -u mainnet-beta --program-id $1 \
    https://github.com/tiago18c/ed25519-programmatic-signer \
    --commit-hash <this commit> --library-name $2 \
    --base-image solanafoundation/solana-verifiable-build:3.1.8 -k "$AUTH"
  solana-verify remote submit-job --program-id $1 --uploader <upgrade authority address>
done
```

## Devnet

The same keypairs deploy the same addresses on devnet (`-u devnet`), so a devnet test of the app runs the code mainnet does.
