# ai-id

AIngle base32 encoding scheme for keys, agents, identifiers, etc.

```rust
extern crate ai-id;

fn main() {
    let enc = ai-id::aiidEncoding::with_kind("ais0").unwrap();
    let key = enc.encode(&[0; 32]).unwrap();
    assert_eq!("aiSciaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", key);
    let buffer = enc.decode(&key).unwrap();
    assert_eq!([0; 32].to_vec(), buffer);
}
```
