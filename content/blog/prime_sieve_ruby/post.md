---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--_-z3_m-s--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/soe9a8yxkf8phwehstqu.jpg
date: 2020-08-06T12:00:00.000Z
title: Prime Sieve In (Hopefully) Idiomatic Ruby
tags:
  - ruby
  - beginners
  - devjournal
  - books
---
Hey y'all.  Been awhile.  Books are at the bottom.

Earlier this week, I got my first "star" on an [exercism.io](https://exercism.io) for a solution very early on in the Ruby track I wrote months ago, so I went back and looked at what I wrote.  I'm far enough removed from writing this solution that reading the code was interesting, and I can tell I was largely eager to flex all the cool toys I just learned.  So, I want to tell you what I did, and I want you to tell me how you'd do it better.

The problem is the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).  I did say it was an early one, but I'd argue problems that are easy to wrap your head around are the best way to flex in a new environment.  That's "era-TOSS-the-knees", if you're like me and read it way before you heard it or said it.  Basically, you can find prime numbers up to `n` by going up a number list one by one, calling each element `x` and crossing out all numbers from `x` up to `n` that are a multiple of `x`.  If it's a multiple of something, it's not prime.  Eventually, the list remaining comprises all the primes from `2` to `n`.  Easy enough.

What's neat, though, is that in Ruby you can kinda just say that:

```ruby
# Sieve of Eratosthenes
class Sieve
  def initialize(n)
    @all = n > 1 ? 2.upto(n).to_a : []
  end

  def primes
    return @all if @all.empty?

    @all.each do |x|
      @all.select { |y| y > x }.each do |z|
        @all.delete(z) if (z % x).zero?
      end
    end
    @all
  end
end
```

The first thing you learn when learning Ruby is how deeply Object-Oriented Programming permeates its design.  Instead of thinking in terms of calling methods and executing functions, which a fundamentally imperative way of approaching a task, it's important to try to shift towards "passing messages" and receiving responses.  The biggest thing I've learned is that design is not so much about the classes you have, it's about how they interface with each other and the messages they can utilize.

Ruby is an idiosyncratic language in that many of its idioms arise from the wealth of messages that are available to pass.  Instead of checking if `2 == 0`, you pass the object `2` - which is an instance of class `Integer` - the message `zero?` and see what it says.  Quite often, I fire up [`pry`](https://pry.github.io/) and enter questions like this:

```ruby
$ pry
[1] pry(main)> 2.methods.grep /z/
=> [:rationalize, :size, :zero?, :nonzero?, :frozen?, :freeze]
```

Cool.  That's all the methods that the number `2` knows how to respond to that contain the letter `z`.  Why?

```ruby
[2] pry(main)> 2.class
=> Integer
```

Right.  Any `Integer` can.  But it goes deeper:

```ruby
[2] pry(main)> 2.class
=> Integer
[3] pry(main)> 2.class.superclass
=> Numeric
[4] pry(main)> 2.class.superclass.superclass
=> Object
[5] pry(main)> 2.class.superclass.superclass.superclass
=> BasicObject
[6] pry(main)> 2.class.superclass.superclass.superclass.superclass
=> nil
```

It's `BasicObject`s [all the way down](https://en.wikipedia.org/wiki/Turtles_all_the_way_down).  If you pass a message to an object, it will first check the object itself for an implementation.  If it isn't found, it will then check the `class` for the implementation (or "go right").  If that class doesn't have it, it will check superclasses (or "go up") until it finds what it needs, or doesn't.  Right one, then up until a match or `nil`.  Easy enough.  Even `Class` is a class (it subclasses `Module`), you can call methods on it like `attr_reader`.  You often see it without parens (`attr_reader :name, :address, :email`), but Ruby doesn't require parentheses for method calls.  It's all "just Ruby".  Whoa.  I highly recommend [Metaprogramming Ruby 2](https://pragprog.com/titles/ppmetr2/) for grokking the significance of all this.

This high degree of runtime reflection is part of what makes Ruby, well, Ruby.  All this to say that the `Integer` class implements the `zero?` method. The visual leap from `2 == 0` to `2.zero?` isn't huge, but the conceptual leap is important.  I am working with a bunch of objects living in memory, and I can ask any of them fairly complex questions about themselves.  Coming from C++ and Rust, where by the time your code executes it's already just "some machine code at some address", it's a big shift in how you understand and work with a running application.  I'm used to writing my own abstractions this way, but not necessarily interacting with the language itself like this.

Which brings me back to the code.  To start off with the sieve, I populate an instance variable `@all` in the class constructor:

```ruby
def initialize(n)
  @all = n > 1 ? 2.upto(n).to_a : []
end
```

When executing the sieve, you have an upper limit for the primes you want to calculate, and this is passed to the constructor as `n`.  I used the tertiary conditional operator to ask if this upper limit is greater than 1.  If it isn't, there's no primes to report, so the fallback at the end is an empty list `[]`.  However, most invocations will pass a larger integer, in which case `@all` gets instantiated with `2.upto(n).to_a`.

Another super idiosyncratic thing about Ruby is how pervasive [`Enumerator`](https://ruby-doc.org/core-2.7.1/Enumerator.html) is.  I find this tendency seeping into the all the code I write, which is why I love learning different types of languages.  The first message we pass `2` is `upto(n)`.  This returns an `Enumerator`:

```ruby
[13] pry(main)> n = 15
=> 5
[14] pry(main)> 2.upto(n)
=> #<Enumerator: ...>
```

For this application, though, I decided to immediately collect the results in an array with `to_a`:

```ruby
[6] pry(main)> 2.upto(n).to_a
=> [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
```

To calculate the primes, the English version was "going up a number list one by one, calling each element `x` and crossing out all numbers from `x` up to `n` that are a multiple of `x`."  Here's the Ruby:

```ruby
@all.each do |x|
      @all.select { |y| y > x }.each do |z|
        @all.delete(z) if (z % x).zero?
      end
    end
```

We'll take that list we just made, `[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]`, and successively call each one `x`.  Then, we look at *every other* element higher in the list, calling it `z`, and delete any in that same list that satisfy `(z % x).zero?`.  If the remainder when dividing `z` by `x` is zero, it's a multiple and therefore not a prime.  Deleting it from the list ensures we won't waste any more work on an eliminated member of the set.

Before we even got to all this, though, there's a one-liner guard clause:

```ruby
return @all if @all.empty?
```

If `@all.empty?` returns true, we know `@all == []`, because arrays can respond to that message.  When that's the case, the empty list does indeed comprise all the primes up to `n`, so pass it on up before moving further.  This is one particular case where I think this code should be improved - I know *in the constructor* that the result will be the empty list.  However, I don't know how the requirements of this class might change (hypothetically), so I decided to not actually return that result until the `primes` message is explicitly passed.  What's your instinct?

However...that's it.  It's more or less what we said in English, but in Ruby.  When we eliminate all the numbers that are a multiple of one smaller, we get the primes:

```ruby
[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
[2, 3, 5, 7, 9, 11, 13, 15] # after 2
[2, 3, 5, 7, 11, 13] # after 3
[2, 3, 5, 7, 11, 13] # None of the rest have any multiples in the set - these are the primes under 15
```

All done.  You can almost read the code like the English:  "take each from @all as x, but specifically select the ones that are higher than x.  Delete it from @all if it's a perfect multiple of x".  And all along the way, each object already knew how to do or answer everything I needed.  All you have to do is ask.

How would you write this simple program?

## Aside

On a more personal note, I've been pretty quiet this pandemic - haven't really known what to write while I shift gears and figure out the path forward, and it's been a bunch of rebuilding fundamentals in a more practical context.  There's been ample book-readin' (for tool-learnin' and ecosystem-familiarizin') and some problem solving on LeetCode-esque-sites (which have thoroughly humbled and redirected me), and some, you know, messing around and such (incoming [chip8](https://en.wikipedia.org/wiki/CHIP-8) post, I got way bit-wiser).  And, like, a bunch of completely non-coding concerns - right?  Who's with me?  2020 can go suck it?

Anyway, I've found it all challenging and necessary, but not necessarily post-inspiring.  However, I've definitely had my mind blown more than once, and these books are to blame:

The Ruby-specific ones:

* [Practical Object-Oriented Design in Ruby](https://www.poodr.com/)
* [Agile Web Development with Rails 6](https://pragprog.com/titles/rails6/)
* [Programming Ruby 1.9-2.0](https://pragprog.com/titles/ruby4/)
* [Metaprogramming Ruby 2](https://pragprog.com/titles/ppmetr2/)

It's tough to recommend any over the other.  They're all quite different, and I'm finding each indispensable.

The Random Other Crap:

* [The Art Of Computer Programming](https://www-cs-faculty.stanford.edu/~knuth/taocp.html) - hardly needs an introduction.  This is slow going, but not one page is a waste.  Put your Assembly cap on.
* [RHCSA 8 Cert Guide](https://www.sandervanvugt.com/red-hat-rhcsa-8-cert-guide-ex200/) - off topic, but also great if this is in your interests.  I also am watching this author's video course, and recommend his Linux instruction material.  Straight and to the point, does not waste your time.  I found I was already closer than I thought, this is perfect for filling in the gaps.  I'm taking [the exam](https://www.redhat.com/en/services/certification/rhcsa) in a few weeks.
* [Calculus and its Applications](https://www.pearson.com/us/higher-education/program/Goldstein-Calculus-Its-Applications-plus-My-Lab-Math-with-Pearson-e-Text-24-Month-Access-Card-Package-14th-Edition/PGM2548008.html) - Also off-topic.  I'm taking Calc 1 in school for the first time, as a 28-year-old would-be engineer, and honestly, it's the most challenging material on here while also blowing my mind.  While there is additional course material, I have largely succeeded in the course by reading this textbook and attempting the homework, as someone who is not adept at math by nature.  Why didn't any of you tell me calculus was cool?

*Cover image: <span>Photo by <a href="https://unsplash.com/@mattseymour?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText">Matt Seymour</a> on <a href="https://unsplash.com/s/photos/strainer?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText">Unsplash</a></span>*
