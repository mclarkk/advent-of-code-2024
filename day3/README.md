# Reflections

Finally an excuse to explore regexes in Rust! I've been looking forward to this.

Part 1 is a very straightforward extraction of multiplication instructions from the source.

In Part 2 I also got to use a min heap, which I've also been looking forward to, since it's a common data structure in these kinds of programming puzzles. 

In Rust it is very easy to get the positions (byte indices) of each regex pattern match. I check to see whether the current location of a multiplication instruction match was most recently preceded by a `don't` instruction or a `do` instruction and then decide whether to do the multiplication based on that.