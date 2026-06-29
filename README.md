# PSHA2.rs

> PSHA2 is a cryptographic hash function specialized for the needs of the google3 build
> system and source system. It is derived from SHA-256 by a tree construction, with 0-4
> intermediate levels depending on message size.
>
> https://docs.google.com/document/d/1JnrL4iFnkWrZUYXiIAZC4KLzOgj0y0gFNd4jZyZT590/edit


PoC in Rust of Google's PHSA2 hash function. Code based entirely on the spec mentionned above and no other (internal) knowledge. This PoC doesn't implement any optimisations, like the ones listed [here](https://eprint.iacr.org/2012/476.pdf) (yet).