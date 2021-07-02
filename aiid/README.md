# ai-id

AIngle base32 encoding scheme for keys, agents, identifiers, etc.

```rust
extern crate aiid;

fn main() {
    let enc = aiid::aiidEncoding::with_kind("ais0").unwrap();
    let key = enc.encode(&[0; 32]).unwrap();
    assert_eq!("aiSciacbd", key);
    let buffer = enc.decode(&key).unwrap();
    assert_eq!([0; 32].to_vec(), buffer);
}
```
