# `gettransaction`

Returns detailed information about a transaction cached in the watch-only wallet database.

## Usage

### Synopsis

```bash
floresta-cli gettransaction <txid> [verbose]
```

### Examples

```bash
# Get transaction details
floresta-cli gettransaction "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"

# Get verbose transaction details
floresta-cli gettransaction "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b" true
```

## Arguments

- `txid` - (string, required) The transaction ID (hex-encoded).
- `verbose` - (boolean, optional, default=false) If true, returns detailed JSON information. If false, returns a similar structure but with default verbosity. Note that in Floresta both verbose and non-verbose settings return a detailed transaction JSON object if the transaction is found.

## Returns

### Ok Response

Returns a JSON object:
- `in_active_chain` - (boolean) Whether the transaction is in the active chain.
- `hex` - (string) The serialized, hex-encoded transaction data.
- `txid` - (string) The transaction ID (hex-encoded).
- `hash` - (string) The witness transaction ID (wtxid, hex-encoded).
- `size` - (numeric) The total transaction size in bytes.
- `vsize` - (numeric) The virtual transaction size in vbytes.
- `weight` - (numeric) The transaction weight.
- `version` - (numeric) The transaction version.
- `locktime` - (numeric) The lock time.
- `vin` - (json array) The transaction inputs:
  - `txid` - (string) The transaction ID of the spent output.
  - `vout` - (numeric) The output index of the spent output.
  - `script_sig` - (json object) The script signature:
    - `asm` - (string) Assembly representation of the scriptSig.
    - `hex` - (string) Hex representation of the scriptSig.
  - `sequence` - (numeric) The input sequence number.
  - `witness` - (json array of strings) Hex-encoded witness stack elements.
- `vout` - (json array) The transaction outputs:
  - `value` - (numeric) The output value in satoshis.
  - `n` - (numeric) The output index.
  - `script_pub_key` - (json object) The locking script:
    - `asm` - (string) Assembly representation of the scriptPubKey.
    - `hex` - (string) Hex representation of the scriptPubKey.
    - `req_sigs` - (numeric) Number of required signatures (deprecated).
    - `type` - (string) The scriptPubKey type (e.g. pubkeyhash, scripthash, witness_v0_keyhash, etc.).
    - `address` - (string, optional) The Bitcoin address associated with this output.
- `blockhash` - (string) The hash of the block containing this transaction.
- `confirmations` - (numeric) The number of confirmations.
- `blocktime` - (numeric) The block time expressed in UNIX epoch time.
- `time` - (numeric) Same as blocktime.

### Error Enum `TxNotFound`

* `JsonRpcError::TxNotFound` - The transaction was not found in the watch-only wallet cache.

## Notes

- This command only returns transactions that are cached or tracked by the watch-only wallet. It does not scan the entire blockchain for arbitrary untracked transactions.
