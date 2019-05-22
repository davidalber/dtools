# dtools
This started out as a little experiment and refresher. I occasionally find good opportunities to use the very cool [data_hacks package](https://github.com/bitly/data_hacks) to visualize data in the terminal. It takes a while for it to chew through sets of data that are 100K or 1M samples large, though. I was curious to see how long it would take to write something similar (starting just with histogram) in Rust and how its performance would compare.

Here are early results on my really old MacBook Pro for `histogram`. Each data point is the average of ten measurements, except the 100M data_hacks value, which is the average of three test runs.

| _n_ samples | data_hacks | dtools  |
| ----------- | ---------- | ------- |
| 10K         | 4.111s     | 0.0135s |
| 100K        | 42.556s    | 0.0488s |
| 1M          | 445.591s   | 0.432s  |

I'll try these tests out on a more powerful machine soon.

## What's Next
Maybe I'll keep going with this. Writing some tests, and then expanding the functionality of the `histogram` subcommand are likely next avenues before expanding into another command. data_hacks has other cool functionality, but I can think of some features -- not directly inspired by data_hacks -- that I'd be interested in exploring.
