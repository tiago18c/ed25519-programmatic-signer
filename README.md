# Ed25519 Programmatic Signer

Long-lived authorizations for Solana. Sign now, submit later.

Solana's blockhash expiration window can make offline signing and collecting multiple approvals difficult. Programmatic
Signer lets authorities sign long-lived messages off-chain, which a relayer can submit later. The programs verify the
signatures, advance a nonce to prevent replays, and invoke the approved instructions via CPI with signer privilege
granted to PDAs derived from the authority's keys. These PDAs hold funds or serve as authorities for token, stake, and
other accounts.

> [!WARNING]
> This is a fork of [solana-program/ed25519-programmatic-signer](https://github.com/solana-program/ed25519-programmatic-signer)
> that runs upstream's programs under its own program IDs, for deployment on mainnet. Upstream is not yet fully
> audited. Addresses derived under these IDs (programmatic signers, the transition signer) differ from upstream's. See
> [MAINNET.md](MAINNET.md) for how to build and deploy.

- **Ed25519 Signer:** `Sig8PHhDifGhptyvrxGRoNNHJSvKYBfSjbxBchTay9E`
- **Message Executor:** `ExeXHWU2yrgUrKQg4QcjqnniW6rWYvqBVP36cq5uwTSp`
- **Nonce:** `NoncFMHL3z7njAbTPMQfscuyg2efjVADwfrhffgBv2h`

## Documentation

See the [Programmatic Signer documentation](https://www.solana-program.com/docs/programmatic-signer) for setup, usage,
migration, and client guides.
