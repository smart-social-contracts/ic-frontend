# ic-frontend

The asset canister Casals installs for stand frontends and its WASM store: a
certified-assets canister that also lets the app keep files of its own.

It is a fork of [dfinity/certified-assets](https://github.com/dfinity/certified-assets)
at `69aaf9c` (May 2026), the last commit with the dfx-era asset API (`store`,
batches, `propose_commit_batch` / `compute_evidence`, permissions). DFINITY's 0.2+
releases are a rewrite with a different interface; this repository does not follow
them, and its versions are unrelated to theirs.

## What it adds

- **Pinned directories.** `pin_directory`, `unpin_directory` and
  `list_pinned_directories`. Assets under a pinned prefix (for example `/ext/` or
  `/custom/`) survive `DeleteAsset`, `UnsetAssetContent` and `Clear`, so files the
  app writes at runtime outlive every frontend deploy. Pins persist across
  upgrades.
- **No 503 after a delete.** Deleting an asset prunes the emptied branches of the
  certification tree, so the deleted path is answered by the fallback page instead
  of a gateway 503 (DFINITY #62, ported).
- `CreateAsset` of an existing key is a no-op instead of a trap, and
  `get_asset_properties` of a missing asset returns defaults.

## Releases

Pushing a `v*` tag builds `ic-frontend@<version>.wasm.gz`. The module carries its
Candid interface (`candid:service`) and its version (`ic-frontend:version`) as
public metadata:

```sh
icp canister metadata <canister> ic-frontend:version -e ic
```

Upgrading a canister that runs the stock DFINITY asset canister (or
certified-assets 0.3.0) to ic-frontend keeps its assets and permissions.

## Crates

- [`canister/`](canister/): the deployable canister.
- [`ic-certified-assets/`](ic-certified-assets/): storage, certification, streaming
  and access control (the crate keeps its upstream name so fixes can be ported).
- [`plugin/`](plugin/): an `icp-cli` sync plugin that uploads a directory through
  the batch API.
