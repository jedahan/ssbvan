# chattervan

A tool for making vanity chattervox keys.

It only **kinda** works, in that the key lengths I get are 93/46, whereas the chattervox ones are 99/49.

This makes me think I have the wrong curve or something. p192 has a few different names, and is not so popular.

Single threaded performance is over 10,000 times faster than the javascript version though.

# benchmarks

|Iterations| Command | Mean [s] | Min…Max [s] |
|:---|:---|---:|---:|
| 100 | `node ~/code/chattervox/build/main.js genvanity --prefix dead --suffix dead` | 79.438 ± 16.144 | 63.652…119.221 |
| 100000 | `cargo run --release` | 6.172 ± 0.106 | 6.068…6.347 |

Summary
  'cargo run --release' ran
   12.87 ± 2.63 times faster than 'node ~/code/chattervox/build/main.js genvanity --prefix dead --suffix dead'

(12.87-2.63)x with 1000x the number of iterations
