Wow, today's challenge was frustrating.
[main.rs.fail](day12_both/src/main.rs.fail) is my attempt at solving this
using iterators.  Apparently if you have nested closures and one is mutable, bad
things happen.  Yeah, in an ideal world, your functional code would be pure and
wouldn't have side effects.  But I thought I'd be able to implement this using
iterators only, but then that didn't scale, and I needed to shoehorn in a cache.
And it didn't go well.

[I posted on stack
overflow](https://stackoverflow.com/questions/74773922/rust-nested-lazy-iteration-and-the-interaction-with-mutable-closures?noredirect=1#comment131971801_74773922)
and the response was overwhelming.  Use loops or don't use mutators.

Anyway, I did end up just putting everything in a loop, and it worked great.
But it's not really my preferred coding style, and it seems like a fairly big
limitations of rust.