# `findtxout`

Attempts to find a transaction output (UTXO) in the blockchain using compact block filters, even if it is not currently tracked/cached by the wallet.

## Usage

### Synopsis

```bash
floresta-cli findtxout <txid> <vout> <script> [height_hint]
```

### Examples

```bash
# Find a UTXO starting the scan from block height 800000
floresta-cli findtxout "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b" 0 "76a91462e905b400ac30f81e85fcfcffc89f54f9b84a1488ac" 800000

# Find a UTXO scanning from genesis (height_hint = 0)
floresta-cli findtxout "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b" 0 "76a91462e905b400ac30f81e85fcfcffc89f54f9b84a1488ac"
```

## Arguments

- `txid` - (string, required) The transaction ID (hex-encoded).
- `vout` - (numeric, required) The output index (vout) to find.
- `script` - (string, required) The hex-encoded locking script (scriptPubKey) of the output.
- `height_hint` - (numeric, optional, default=0) The block height from which to start scanning the compact block filters. Providing a hint close to the block height where the transaction was mined significantly reduces scan times.

## Returns

### Ok Response

Returns a JSON object representing the UTXO details (if found):
- `best_block` - (string) The hash of the best block tip.
- `confirmations` - (numeric) The number of confirmations.
- `value` - (numeric) The output value in BTC.
- `script_pubkey` - (json object) Information about the output script:
  - `asm` - (string) Assembly representation of the scriptPubKey.
  - `hex` - (string) Hex representation of the scriptPubKey.
  - `descriptor` - (string, optional) Descriptor with checksum.
  - `address` - (string, optional) The associated Bitcoin address.
  - `type` - (string) The script type (e.g. pubkeyhash).
- `coinbase` - (boolean) Whether the output was created in a coinbase transaction.

If the UTXO is not found, an empty JSON object `{}` is returned.

### Error Enum

* `JsonRpcError::InInitialBlockDownload` - If the node is currently in Initial Block Download (IBD) mode.
* `JsonRpcError::NoBlockFilters` - If compact block filters are not enabled (`blockfilters=1` option).

## Notes

- This is a Floresta-flavored RPC method and is not part of the standard Bitcoin Core RPC interface.
- It scans the compact block filters (`BIP 157/158`) matching the provided `script`. If there is a match, the node downloads the corresponding block, processes it to update the wallet state, and retrieves the UTXO details.
- To use this RPC, you must have enabled block filters by setting the `blockfilters=1` option in your configuration.
