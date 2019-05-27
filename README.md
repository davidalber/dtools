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

# Commands
## Histogram
Given a column of data on stdin, this command renders the data as a histogram. Example:

```
$ for i in {1..1000000}; do echo $RANDOM; done | dtools histogram
# NumSamples = 1000000; Min = 0.00; Max = 32767.00
# Mean = 16376.431793; Variance = 89525281.435739; SD = 9461.780035; Median = 16367.000000
# each ∎ represents a count of 1338
    0.0000 -  3276.7000 [ 100233]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 3276.7000 -  6553.4000 [ 100375]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 6553.4000 -  9830.1000 [  99950]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 9830.1000 - 13106.8000 [  99495]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
13106.8000 - 16383.5000 [ 100417]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
16383.5000 - 19660.2000 [  99597]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
19660.2000 - 22936.9000 [ 100057]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
22936.9000 - 26213.6000 [  99964]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
26213.6000 - 29490.3000 [ 100321]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
29490.3000 - 32767.0000 [  99591]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
```
