# aba12c
Rust implementation to ABA12C problem at spoj

Compilation Error (Should be fixed at version 1.40)
```
error[E0658]: use of unstable library feature 'try_from' (see issue #33417)
 --> prog.rs:1:5
  |
1 | use std::convert::TryInto;
  |     ^^^^^^^^^^^^^^^^^^^^^

error[E0658]: use of unstable library feature 'try_from' (see issue #33417)
  --> prog.rs:13:26
   |
13 |             price: price.try_into().expect(""),
   |                          ^^^^^^^^

error[E0658]: use of unstable library feature 'try_from' (see issue #33417)
  --> prog.rs:24:45
   |
24 |                 let price: usize = (*price).try_into().expect("");
   |                                             ^^^^^^^^

error[E0658]: use of unstable library feature 'try_from' (see issue #33417)
   --> prog.rs:252:22
    |
252 |     return (input[0].try_into().expect(""), input[1].try_into().expect(""));
    |                      ^^^^^^^^

error[E0658]: use of unstable library feature 'try_from' (see issue #33417)
   --> prog.rs:252:54
    |
252 |     return (input[0].try_into().expect(""), input[1].try_into().expect(""));
    |                                                      ^^^^^^^^

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.
```
