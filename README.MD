# Rust KVaC Implementation

This is a Rust implementation of Key-Value Commitments (KVaC) as described in ["KVaC: Key-Value Commitments for Blockchains and Beyond"](https://eprint.iacr.org/2020/1161.pdf) by Shashank Agrawal and Srinivasan Raghuraman.

## Overview

KVaC is a cryptographic primitive that enables efficient commitments to key-value pairs with support for:
- Inserting new key-value pairs
- Updating existing values
- Generating and verifying proofs of membership
- Batch updates for multiple operations

## Features

- ✅ Key-value pair insertion with proof generation
- ✅ Value updates with proof updates
- ✅ Proof verification
- ✅ Thread-safe public parameters using `once_cell`
- ✅ Built on unknown-order group operations

## Dependencies

- `unknown_order`: For cryptographic group operations
- `once_cell`: For lazy initialization of public parameters
- `sha256`: For hash function implementation
- `rayon`: For potential parallel processing
- `criterion`: For benchmarking (dev dependency)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-kvac = "0.1.0"
```

## Usage

### Basic Example

```rust
use rust_kvac::{ insert::insert, params::*, verify::verify, update::update, proof_update::proof_update,};
use unknown_order::BigNumber;

// Access the public parameters
let params = &*PUBLIC_PARAMS;

// Create initial commitment
let commitment = Commitment::new(params.one.clone(), params.g.clone());

// Insert a key-value pair
let kv = KeyValue::new("key1".to_string(), BigNumber::from(42));
let (new_commitment, proof, op) = insert(&commitment, &kv);

// Verify the proof
let result = verify(&new_commitment, &op, &proof);
assert!(result.is_ok());
```

### Full Example

```rust
use rust_kvac::{ insert::insert, params::*, verify::verify, update::update, proof_update::proof_update,};
use unknown_order::BigNumber;

// Access the public parameters
let pp = &PUBLIC_PARAMS;

// Initialize the initial commitment
let commitment = Commitment::new(pp.one.clone(), pp.g.clone());

// Define keys and values
let kv1: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(8));
let kv2: KeyValue = KeyValue::new("test".to_string(), BigNumber::from(10));
let kv3: KeyValue = KeyValue::new("test".to_string(), kv1.value() + kv2.value());

// Insert the first key-value pair
let (commitment2, proof, kv1) = insert(&commitment, &kv1);

// Verify the first insert
let verify_insert = verify(&commitment2, &kv1, &proof);

// Update the first key-value pair
let (commitment3, kv2) = update(&commitment2, &kv2);

// Update proof_k1 in place
let proof_update = proof_update(kv1.key(), &proof, &kv2).unwrap();

// Use the updated proof
let verify_update = verify(&commitment3, &kv3, &proof_update);
```

## Testing

Run the test suite:
```bash
cargo test
```

Run benchmarks:
```bash
cargo bench
```

## Implementation Details

- Updates are additive
- Uses the `unknown_order` crate for group operations in RSA groups
- Implements the hash-to-prime mapping as described in the paper
- Provides thread-safe access to public parameters
- Includes comprehensive test coverage for all operations

## Security Considerations

This implementation:
- Uses 1024-bit RSA groups (configurable)
- Implements secure hash-to-prime mapping
- Follows the paper's security requirements for group operations

## ⚠️ Security Warning

This implementation is a research prototype and has not undergone a comprehensive security audit. It should not be used in production environments without proper review and auditing. Known considerations:

- The implementation needs formal security verification
- No external security audits have been performed
- May contain unknown vulnerabilities or implementation flaws
- Cryptographic implementations require expert review
- The codebase is still under active development

We welcome security researchers and cryptography experts to review and audit the codebase. Please report any findings through our security policy or by opening an issue.

USE AT YOUR OWN RISK. THE AUTHORS AND CONTRIBUTORS ARE NOT RESPONSIBLE FOR ANY DAMAGES OR SECURITY ISSUES ARISING FROM THE USE OF THIS CODE.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

Copyright (c) 2024 Jean Gal

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Citation

This is an implementation of the KVaC protocol described in the following paper. If you use this implementation, please cite both this repository and the original paper:

The original paper:

```bibtex
@misc{cryptoeprint:2020/1161,
    author = {Shashank Agrawal and Srinivasan Raghuraman},
    title = {KVaC: Key-Value Commitments for Blockchains and Beyond},
    howpublished = {Cryptology ePrint Archive, Paper 2020/1161},
    year = {2020},
    url = {https://eprint.iacr.org/2020/1161}
}
```

This implementation:

```bibtex
@software{rust_kvac_impl,
    author = {Jean Gal},
    title = {Rust Implementation of KVaC Protocol},
    year = {2024},
    url = {https://github.com/jjeangal/rust-kvac},
    note = {Implementation of KVaC protocol by Agrawal and Raghuraman}
}
```

## References

- Original KVaC Paper: [KVaC: Key-Value Commitments for Blockchains and Beyond](https://eprint.iacr.org/2020/1161.pdf)
- This implementation is based on the algorithms and protocols described in the paper above