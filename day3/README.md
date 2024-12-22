# Reflections

Finally an excuse to explore regexes in Rust! I've been looking forward to this.

I have not attempted Part 2 yet, but it doesn't look so bad. In Rust it is very easy to get the positions (byte indices) of each of the pattern matches. I would expect to do something like check to see whether the current location of a multiplication instruction match was most recently preceded by a `don't` instruction or a `do` instruction and then decide whether to do the multiplication bit after that.