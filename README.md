# Heavy Duty Bools

Heavy Duty Bools are the more robust cousins of regular `bool` values
in Rust. While both normal bools and Heavy Duty Bools each take up 
only 8 bits of memory, Heavy Duty Bools come with much stronger
recoverability in the event of bit flips. 

This is due to Heavy Duty Bools being `u8` values where all the 
bits in the value are set to the same thing.

**Normal Bools**

True: `0b00000001`

False: `0b00000000`

**Heavy Duty Bools** 

Heavy Duty True: `0b11111111`

Heavy Duty False: `0b00000000`

With the Heavy Duty version, if I change any bit at random, the 
system can still figure out which Heavy Duty value the overall 
value is by calling the `refresh_hdbool()` function. 

On the other hand, normal bools don't have any level of inherent 
redundancy, as only the final bit determines the value of their 
entire unit. 

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
heavy_duty_bools = "[VERSION NUMBER]"
```

Add this to your crate root:

```rust
use heavy_duty_bools::*;
```

