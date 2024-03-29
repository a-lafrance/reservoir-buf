# reservoir-buf
Dead simple interning in Rust

**NOTE: This is a very early-stage project that for now I'm mostly only using in personal stuff**

# Overview

After having worked on several ground-up compiler projects in Rust, I find myself writing pretty much the same code over and over in a few different places. One such case was interning: I found the "store everything in a central buffer and pass around indexes into it" approach to be a good way to represent highly-interconnected data structures (like, say, a control flow graph). Rather than write the same thing again for the n-th time, I figured I'd stick it in a separate crate I could just use everywhere (and hey, maybe other people have a similar need).

The quick and dirty prototype I sketched out captures pretty much the same level of sophistication as what I've used in those projects (with a bit more type safety): store everything sequentially in a big `Vec` and pass around indexes into that buffer which can then be used to interact with the actual data. This is basically a super watered-down form of interning (mostly missing the deduplication but possibly more) which also bears a passing resemblance to an arena allocator. But it does what it promises and gets the job done for me at least.

In the near future I'll be using this questionably put-together version of the crate in my own projects, so I can refine it for a transition into a crate that's ready for wider consumption. Over time I'd also like to improve it to push it toward something that's useful as a generic interner data structure in more sophisticated projects, compilers and otherwise.
