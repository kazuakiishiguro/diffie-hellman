# Diffie Hellman

A portable DH key exchange toy-example.
:warning: WARNING: This library has not been audited, so please do not use for production code.

## Example

```rust
use diffie-hellman::{private_key, public_key, secret};

let p: u64 = 11;

// Assume Alice want to share secret with Bob
let priv_a: u64 = 7; // Alice's private key
let pub_b: u64 = 8; // Bob's public key

// Share secret
let secret = secret(p, pub_a, prib_b);
let expected: u64 = 2;
assert_eq!(secret, expected);
```

## License

License under

* [MIT license](http://opensource.org/licenses/MIT)
