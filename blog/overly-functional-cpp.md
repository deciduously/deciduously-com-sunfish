---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--hXACwaGL--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/iapx6qiql3k0z3beelsb.jpg
edited: 2019-10-02T12:00:00.000Z
title: Overly Functional C++
published: true
description: A reflection on learning C++
tags: beginners, cpp, functional, devjournal
---
# Or Just Functional Enough?

Happy [5780](https://www.hebcal.com/holidays/2019-2020), DEV!

I wanted to see if I could write a [fold function](https://en.wikipedia.org/wiki/Fold_(higher-order_function)) with what I've learned so far about C++.  Clearly, the afternoon got away from me.

This is a three-part post.  This first post is largely a personal reflection.  You may also know this style as a "ramble" or "rant".  The second post is by contrast brief and focused, and discusses the small C++ experiment I mention above from a merely technical standpoint.  The final post makes it generic for collections of `std::vector<T>` and demonstrates several library functions defined in terms of that fold template.  Both parts stand alone, so feel free skip either (or, you know, both, I'm not your dad).  Not everyone likes their code examples served with a side of marginally-relevant ranting, and who am I to judge.  Pick your poison.

## The Situation

I started by quickly jotting down what I thought this function might resemble for input collections of `std::vector<int>` in about five minutes, just to see how far that got me.  As it turns out, after a little extra research, I hit the mark with the first successful compile.  I am still quite a C++ beginner, so a working implementation of *anything* that quickly is an achievement for me.  This was something I hadn't tried and used a library feature I'd never tried, so I had emotionally prepared myself for a small-to-medium battle to even this point before sitting down.  To have a working C++ implementation pop out just a few minutes later initially surprised me.

For those keeping score, here is precisely where I got completely derailed.  Upon further reflection, I reasoned it shouldn't have surprised me at all, and made the mistake of wondering why it did anyway.

## Modern And Hard

Even though I've been building actual hands-on experience with the language lately, I still harbor an outdated preconceived notion about what using C++ is like.  It's at odds with my experience, but the fact that it *actually still is hard* in some ways makes it even tougher to shake this notion.  I do sometimes struggle writing correct and complete C++ after I've "solved" a problem in the abstract sense.  It's probably the hardest language I have tackled to date, but somewhere along the line I had conflated "hard" with "archaic".  C++ is a large language and you actually do need to at least somewhat understand a significant portion of how it works in order to not write horribly broken code, which is hard.  Lots of languages are large, but let you write more simple programs without getting exposed to what you don't need until you need it.

My conclusion about my disconnect here is that it's a semantic problem, and I'm pretty interested by that implication.  I somehow got the idea that "modern" in this context just meant "relative to really old" and "modern C++" was still in essence the same beast it's always been with some new bells and whistles, ultimately archaic compared to other languages I had spent time building comfort with, so necessarily more annoying and verbose to use but worth the effort.  When used to describe "C++", I heard this invented connotation without actually knowing the first thing about it.

Now armed with, I don't know, *anything concrete* to form an opinion from, I disagree with that characterization.  I wish I had taken the time to examine it critically in myself earlier, because it's subconsciously diminished C++ in my mind when in fact I think I always would have enjoyed it.  What's fascinating, though, is that even though my current experiences contradict it and I rationally have worked out that I feel differently, I still initially expect all new C++ problems to be complicated at the outset even when I know how to solve them using the tools provided.

### What's Modern?  What's Hard?

It's important to precisely define what I mean by these terms even when making wild blanket statements about them, because while they may be irresponsible generalizations, at the very least I want them to mean more or less what I think they mean.

#### Defining Modern

Modern is a little easier, because I think it's actually a pretty meaningless term and any specific definition does not matter.  Whatever the working definition, it means the same thing for C++ as it does for Java or Golang or whatever.  Written out this observation is obvious on its face, and I'm unsure if my personal connotation was completely made up or I learned it from somewhere.  Either way it was not based in facts, and either way I held it for over a decade.

#### Defining Hard

Defining what makes a language "hard" is inherently, well, hard.  Most definitions are partially or wholly subjective.  While that's an interesting topic as well, in this context I am specifically attempting to keep my definition as objective as I can.

In C++, you have everything you need at your disposal to very precisely and explicitly define what you need to have the computer do at a highly granular level.  It's also almost comically easy when learning C++ to write *almost* what you mean but not quite, not spot the difference, and get unexpected results that can sometimes differ significantly from what you thought you were getting.  Debuggers can help you explore what you wrote, but not what you meant to write - you still need to understand a lot of implicit machinery to sleuth out how you got what you did.  This happens to me more often as a beginner in C++ than it has learning anything else to date.

The remedy for this as far as I can tell is knowledge - the root of the problem is usually something I had either not been taught yet, forgotten about, or not spotted, but the tooling had allowed.  Of course, this process is part of learning any new language.  In C++, though, the frequency of the problem, the complexity of the issues encountered at an early stage, the subtlety of their syntactic manifestations, the background knowledge required to understand these issues and their solutions, and the lack of or misleading guidance from the tooling all help raise the threshold for a productive output vs troubleshooting/debugging ratio.

Put another way, at some point you have gained sufficient knowledge to use the language to complete work and create value in a timely fashion, that also won't ultimately lose value by messing up all over the place and needing to be fixed or replaced.  Gaining that knowledge is a pretty individual process, but it's generally proportional to work put in.  If that threshold for language A is higher than for language B and takes most devs more energy to achieve, I'm calling language A harder.  Strictly by that definition C++ is the hardest thing I've personally come across.

My perception is that even if you become an expert in C++ it still objectively has a relatively high amount of language-level friction ("hardness"), but it feels built from parts that themselves individually excel.  You need a high level of understanding of what all the various parts are and what they do in order to use this language effectively and safely, but becoming an expert in any language requires that.  C++ starts hitting you in the face with it much faster, but also gives you all these different, powerful ways to keep yourself safe and sane while writing the code you need.

#### Hard != Bad

This wealth of features to learn exists for a reason, and it's hard to imagine a problem for which C++ is completely unsuited.  C++ has been the first tool I've learned that actually feels like it imposes around the same amount of friction regardless of the paradigm you choose to approach a solution.  You're driving a manual no matter what, so just say what you need.  It's my "desert-island" pick - if somehow for the rest of my life I only ever can use one programming language for every new task, at least from what I've learned C++ is a no-brainer.

#### Discuss: The Weird Stuff

Today I'm specifically exploring functionally-flavored C++, but there is [a zoo](https://cs.lmu.edu/~ray/notes/paradigms/) of other interesting programming paradigms out there.  My gut feeling is that C++ could be a viable, if not always optimal, choice for exploring almost any of them.  I would be curious to hear your experiences or why that is or isn't actually the case, or if you have a specific counterexample.  Tried logic programming in C++?  How'd that go?

### The C++11 Experience

I thought in tackling C++ I had signed up for some good old-fashioned Object-Oriented concepts and design patterns, and to be sure I have received no shortage of that stuff so far.  I knew the language had received some updates since I'd used it pre-2010, though, but I didn't understand the details of what those updates contained.  Not to give too much away, but "some updates" is an egregious understatement.

C++11 brought with it, among many other additions, ergonomic, built-in support for first-class functions through [lambdas](https://en.cppreference.com/w/cpp/language/lambda) and a huge set of explicit tools to ensure other needed traits such as immutability.  C++ compilers have also optimized for tail-call recursion since even before that, which enables a larger set of recursive solutions to be pragmatic as well as concise.  [Templates](https://en.cppreference.com/w/cpp/language/templates), not a new feature, up the flexibility even further with what you can concisely define.  Pick a paradigm,  C++ is likely flexible enough to do what you ask provided you've got the chutzpah.

Along with all these changes has come a new set of idioms that was not previously possible with standard C++ which have overhauled both how C++ is written and taught.  While I can't vouch for everything, generally the beginner material I have seen has taught using the new features and idioms from the beginning, and introducing standard classes like "vector" and "string", instead of the older C-With-Classes type code I had started from before, which is a lot easier and less frustrating to get started writing real solutions with.  You still need to learn the fundamentals of what a C-string is, but there's no reason you can't immediately start benefiting from the safer and more flexible `string` API once you do.

The details of everything this standard adds could be its own series.  To name-drop a few C++11 goodies beyond lambdas I've immediately started leveraging are things like [range-based for-loops](https://en.cppreference.com/w/cpp/language/range-for), [type inference with `auto`](https://en.cppreference.com/w/cpp/language/auto), derived class constructor [delegation syntax](https://www.ibm.com/developerworks/community/blogs/5894415f-be62-4bc0-81c5-3956e82276f3/entry/introduction_to_the_c_11_feature_delegating_constructors?lang=en), the [`nullptr`](https://en.cppreference.com/w/cpp/language/nullptr) constant, [`enum class`](https://en.cppreference.com/w/cpp/language/enum) syntax, and new UTF [string literal](https://en.cppreference.com/w/cpp/language/string_literal) syntax.  I also appreciate the curly-brace initialization syntax, having never managed to keep the nuances of that topic straight before.

That's just the core language stuff, too, and the standard library also expanded significantly and plugged some much-needed holes.  Pulling in third-party dependencies is still a friction point for me, so the more I can lean on the standard distribution the better.  As before, I've only partially explored these additions, but I've already made use of tuples and smart pointers like [`shared_ptr`](https://en.cppreference.com/w/cpp/memory/shared_ptr).  Another important addition I haven't used myself is the [atomic operations](https://en.cppreference.com/w/cpp/atomic) library for writing threaded code.

Basically, it feels like using any other modern programming language I've tried.  In retrospect this really should not be surprising to me.

### And, Like, It's Almost 2020 Now

This revision was approved on August 12, 2011.  I ran the numbers, and that's actually over eight years ago now.  Granted, in some domains that's not a significant time frame, but even in slow-moving worlds there's been a lot of time for the new stuff to percolate.  This revision maintains backwards-compatibility with all existing legacy-style code, but the set of new features in this version added represent an entirely new set of idioms for writing new C++ code that focus on memory safety, performance, and ergonomics.  It almost feels like a different language than what I had started tackling the very first time around.

Since that revision, we've again received new and powerful changes and tools with C++14 and C++17.  C++20 is even bigger than the previous two, more on par with C++11 in scope, as it pulls together and finalizes some long-term planned work as well as new ideas.  It's within a year of becoming the new official standard.  I've barely looked at *any* of this new stuff myself yet but some of it does bring us even more functional-flavored goodies - for a single cherry-picked example, C++14 makes [lambdas even more powerful](http://open-std.org/JTC1/SC22/WG21/docs/papers/2018/p1319r0.html) with features like polymorphic lambdas and lambdas with default arguments, among others.

## Prior Experiences

Using C++ so far reminds me distinctly of two prior self-learning experiments, namely Common Lisp and Scala. Each is for a slightly different reason.  I believe even the little context I have from trying each has proved directly helpful in approaching C++ now, and I'm curious now to revisit both and see if the reverse will also be true.

Interestingly, I chose to attempt both of the above languages because I had learned Clojure and highly enjoyed many but not all aspects of working with it. That's a language with very little in common with C++, except perhaps in degree of versatility.  Clojure wasn't without its complexity and pitfalls for a novice, though, and not necessarily an exact fit for any of the code that I wanted to write, though, so I wanted to explore some other similar options to see if anything stuck better.  C++ was unfortunately not even on my radar at the time as a contender, which is a shame.

I'm including [Rust](https://www.rust-lang.org/) as an honorable mention.  I don't personally feel these two are that similar, but some of those differences seem to be precisely what motivated Rust to exist in the first place which is also interesting.

### Multi-Paradigm: Common Lisp

[Common Lisp](https://common-lisp.net/) is also a truly multi-paradigm programming language.  It's like a gigantic kit of all these different tools you can use, and really has few-to-zero opinions about how you use them and so little syntax it's all pretty easy to use once you know the basics.  This is at least at the language level - the complexity is all at the library level.  Common Lisp sometimes gets shoehorned into a category with the functional-forward [Scheme](http://www.r6rs.org/)-family lisp-alikes for cosmetic reasons, but it's really not an accurate grouping.  In fact, the Common Lisp Object System is "arguably one of the most powerful object systems available in any language."  True, this quote is [according to the people that wrote the book](https://lispcookbook.github.io/cl-cookbook/clos.html), but it's precisely Common Lisp's flexibility that makes this statement defensible.  You can choose to use the CLOS or not even touch it, but it's there for you along with everything else.

Like C++, this property makes the language extremely powerful and due to different design trade-offs is also capable of dramatically reducing development time.  Like C++, though, it is also full of footguns and idiosyncrasies.  "Powerful" and "Easy to use" have a tendency to be tough to package up together, because adding power generally involves adding complexity in one way or another.  This is especially true when you add a few decades of history to a "kitchen-sink"-style mentality.

This style language is most useful if you already know pretty well what you're doing, what you want, and how to get there, so you don't want your language to tell you want you can and can't do.  I still plan to come back someday, but I generally don't know any of those things nearly well enough yet.  Notably, CL has been this feature-rich for decades before C++11 rolled along, and still maintains a small but active and dedicated community today.  It's not going anywhere any time soon.

### Functionally Object-Oriented: Scala

I tried Scala to get a feel for a different [JVM](https://www.javaworld.com/article/3272244/what-is-the-jvm-introducing-the-java-virtual-machine.html) language having spent a while learning [Clojure](https://clojure.org/).  In Clojure you can largely stay within the environment provided by the language to get up and running, but you can't avoid it's fundamental nature as a hosted language by design - interop with the host platform comes up quickly even for beginners.  Standard Clojure is hosted on the JVM, and I didn't know anything about the JVM yet.  ClojureScript, which basically just Clojure but hosted on JavaScript instead, was a lot easier for me at first because I understood more or less how the underlying platform worked.  A Java stacktrace was completely foreign every time I misused a Java collection type.

I also had seen Scala compared to [Haskell](https://www.haskell.org/) on multiple occasions in functional programming spaces, which I also already had spent time getting comfortable with.  That's where I'd heard of it in the first place, and this similarity sold me over just going for Java.  I gravitate towards learning tools with some direct overlap to what I already know, and I really didn't know anything about OOP but did know something about fancy type systems.

I didn't get very far, because it was quickly apparent to leverage Scala effectively a healthy understanding of Java is useful first.  In a similar manner, to fully and correctly leverage everything C++ has to offer you need a healthy understanding of, well, C++.  From what I did take away, though, it feels like C++ itself covers a lot of Scala's raison d'être over Java just fine.  You don't get the same level of sophistication in your type system, but if the concern is static analysis isn't the net gain for a team comparable?

Scala seems suited to applications that intend to blend functional and object-oriented implementation approaches, as opposed to the more standard imperative-flavored OOP.  I don't see why C++ can't be used for this as well, at least for non-academic use-cases that aren't doing type-level research.  I may be over-simplifying the pattern, but my understanding is that you use classes to provide large-scale structure but use functional method definitions over imperative and stop coupling all your data to your logic like OOP encourages.

#### Discuss: Scala-flavored C++?

This question isn't about the merits of managed vs. unmanaged languages or Scala vs C++ in any specific actual context, but rather about general language analogues.  Scala is a reaction to Java within that ecosystem designed to meet a perceived unmet need, and my hypothesis is that post 2011 C++ is already adequate in the ways the Scala folks perceived Java not to be.

Now, Scala predates C++11, and Java has also undergone large changes which also address some of these original shortcomings.  Basically, whether this statement is true or not has no bearing on reality.  I'm just curious if my perception is accurate that C++ is suited for the style of program that Scala was designed to facilitate.  I will be first to admit this is not a a well-informed observation, just a beginner's perception, does anyone who actually knows more have an opinion on this?

### Honorable Mention: Rust

Rust is currently my favorite and most-used language.  Each new project I write in Rust is teaching me something I hadn't come across before about how to write Rust well, and every time I just like the language even more for it.  It's still a niche tool and will likely remain so for a long time if not forever, but at least for personal work I've yet to find something that pushes more of my buttons.

 Rust targets some of the same domains where C++ currently dominates, but the subjective experience of building a program in each feels much more dissimilar to me than to either of the above.  This is extremely deliberate on the part of the Rust folks, and has also been eye-opening coming from the other side.  C++ has demonstrated quite clearly why exactly some benefits I already abstractly understood Rust provided to me were useful.  Once, what I thought was a correctly-implemented medium-sized program randomly segfaulted on some innocuous test input.  It wasn't immediately obvious to me why it was broken and somehow almost offended me as a knee-jerk reaction.  Like, a segfault?  What is this, the Triassic?

No, as it turns out, it wasn't the Triassic, it was just my fault, as usual.  I had written bad code that does a bad thing with some memory that wasn't mine to touch, and it's *my* job to fix that problem and not write such code again.  I'm completely spoiled in Rust, because even though you can still write bad or logically wrong Rust, it won't ever let you make that mistake outside of `unsafe`, so I never thought about how easy it really is to introduce subtly without that check.

### ¿Porque no los dos? - C++

I ultimately didn't spend long learning either language, eventually outgrowing my language-hopping phase in favor of writing actual software, and fast-forward to now I have definitively chosen C++ as the "industry tool" I want to focus on and hone.  I always knew I'd find it useful, but I had not anticipated how much I'm actually enjoying it.  As I learn more I enjoy it more, and expect it to become one of my most-used languages for personal projects as well eventually.  There is still a lot to know, and a lot of ways to hurt yourself like I remember, but I was expecting to step back into the dark ages to get anything done in terms of expressiveness.  This has not at all been the case.

You, C++ expert, probably could have filled me in on this, it's on me for never asking.  And, to be sure, C++ is not without it's drawbacks as well.  Like Common Lisp, the programmer bears a relatively higher portion of the burden of program structure, correctness, and soundness than with other languages.  Developers need both high knowledge and discipline to write safe, effective software, and other available stack choices may offload more of that to the computer without sacrificing functionality in your domain.  If that's the case for your project, of course you should go that route.  In terms of sheer versatility, though, C++ is selling itself to me pretty dang hard.

*Photo by Michał Parzuchowski on Unsplash*
