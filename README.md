# lcs_rs

[Longest common subsequence](https://en.wikipedia.org/wiki/Longest_common_subsequence) implementation in rust (and Java).

## bench for the 10000-10000 test (the 15000 one doesn't work):

### Rust

```
Time: 687.737625ms
Time: 713.694264ms
```

### Java

```
Time: 623.69858 millis
Time: 668.713163 millis
```

This result varies ~ += 50 ms on my pc.

None of both implementations have been hardly optimized, but the algorithm used is quit fast.

## Attempt to explain why rust is slower (please don't quote me on this)

Maybe the jvm allocate some space directly when launching the program, which make the allocation of the matrice faster.

Rust need to iterate over all chars to get the number of chars in the string, because an UTF-8 can have different size. I don't think Java does this.
