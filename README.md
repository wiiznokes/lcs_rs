# lcs_rs

Longest common subsequence implementation in rust.

https://en.wikipedia.org/wiki/Longest_common_subsequence


bench for the 10000-10000 test (the 15000 one doesn't work):

##### Rust
Time: 677.810401ms
Time: 713.694264ms

##### Java
Time: 623.69858 millis
Time: 668.713163 millis

This result varies += 50 ms on my pc.

None of both implementations have been hardly optimized, but the algorithm used is quit fast.