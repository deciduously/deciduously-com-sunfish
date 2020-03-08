---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--ExRsJt-e--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/raswv0kgmynr76u653kj.jpg
edited: 2020-03-07
title: Getting Cozy with Ruby
published: true
tags: beginners, ruby, devjournal, tutorial
---

When I first approached Ruby, I basically looked at it like "dynamic C++", because that was the best analogue I had.  Of course, that has required some tweaking.  This post talks about some Ruby-specific idioms that aren't directly related to concepts I knew, and assumes you already use another class-based OOP language such as C++, Java, or Python.

Now that I've been using Ruby for about three days, I'm obviously a complete expert and to be implicitly trusted (heh).  If you do see something wrong, please correct me!

Huge thanks to `@dvik1950` on the Ruby [exercism](https://exercism.io) mentored track for many of these.

I know this splash image only mildly applies but it was *too cool* not to use.  You can't change my mind.

## Testing

This is a dynamic language.  I am so very much a static kinda person, so unit testing is pretty much the first priority for not losing hair/sleep.

I found the easiest to start with is [minitest](https://github.com/seattlerb/minitest):

```
$ gem install minitest
```

Then, in `my_math_test.rb`:

```ruby
require 'minitest/autorun'
require_relative 'my_math'

# MyMath sanity tests
class MyMathTest < Minitest::Test
  def test_times_two
    # skip
    assert_equal 8, MyMath.my_times_two(4)
  end
end
```

Uncomment `skip` to skip the test, which avoids commenting/uncommenting the whole function.  Also, the `MyMathTest < Minitest::Test` syntax is how you define a subclass, so `MyMathTest` inherits from `Minitest::Test`.

For a much more involved solution, there's [`rspec`](https://rspec.info/) which provides a testing DSL instead of using Ruby functions.  Here's what that test might look like:

```rust
describe MyMath do
   it "multiplies 4 by 2" do
     math = MyMath.new
     expect(math.my_times_two(4)).to eq(8)
  end
end
```

This reminds me of using [Mocha](https://mochajs.org/) with [Chai](https://www.chaijs.com/) in JavaScript.  There's an intro guide [here](https://www.rubyguides.com/2018/07/rspec-tutorial/).

## percent-w

Don't do this:

```ruby
JOES = ["average", "DiMaggio", "morning"]
```

Do this:


```ruby
JOES = %w[average DiMaggio morning]
```

Whoa!  Also, `%i` works for [symbols](https://ruby-doc.org/core-2.2.0/Symbol.html):

```ruby
SYMS = %i[one two three]
# [:one, :two, :three]
```

## Object.freeze

Not planning to ever change your `JOES` constant?  Tell Ruby that you mean it and freeze 'em:

```ruby
JOES = %w[average DiMaggio morning].freeze
```

Now it's actually immutable!  Read a lot more about Ruby constants [here](https://www.rubyguides.com/2017/07/ruby-constants/).

## String interpolation

Basically, it does it.  This works:

```ruby
def say_name(name)
  output = "Hello, "
  output << name
  output << "!"
  puts output
end
```

This is better:

```ruby
def say_name(name)
  puts "Hello, " + name + "!"
end
```

But you probably want **this**:

```ruby
def say_name(name)
  puts "Hello, #{name}!"
end
```

## class << self

To redefine a method on `self`, so you can call `MyClass.my_method`, you can define it on `self` explicitly:

```ruby
class MyClass
  def self.my_method(str)
    puts str
  end
end
```

If you're doing this a bunch, you can open up the *eigenclass*, or singleton class, directly:

```ruby
class MyClass
  class << self
    def my_method(str)
      puts str
    end
  end
end
```

I think that's a little less noisy, even at the cost of some extra lines and indentation.  If you want to go super concise, you can just dot-operator your way all the way in:

```ruby
class MyClass
end

def MyClass.my_method(str)
  puts str
end
```

This  `class << self` is also the best way to make a private method:

```ruby
class MyClass
  class << self
    private

    def my_private_method(str)
      puts str
    end
  end
end
```

Otherwise, you have to use `private_class_method` which I think looks gross:

```ruby
class MyClass
  private_class_method def self.my_private_method(str)
    puts str
  end
end
```

## attr_accessor

You can publicly expose instance variables directly:

```ruby
class MyClass
  def initialize
    @value = 0
  end
  def show_value
    puts @value
  end
end
```

You define the constructor with `initialize()`.

In Ruby, however, it's extremely easy to create getters and setters and usually preferable.  You can manually do so:

```ruby
class MyClass
  def initialize
    @value = 0
  end

  def value
    @value
  end
  def value=(new_value)
    @value = new_value
  end

  def show_value
    puts value # don't point to the `@value` var, send the `value` message
  end
end
```

It's usually better to create both at once with `attr_accessor`:

```ruby
class MyClass
  attr_accessor :value

  def initialize
    @value = 0
  end

  def show_value
    puts value
  end
end
```

You can also use `attr_reader` or `attr_writer` for just the getter or just the setter, respectively.

The benefit is that now if this logic needs to change, all you need to do is define that method, and every call site automatically reflects the new logic.

## Structs

You don't have to use nested arrays and whatnot for complex data just because we're using a dynamic language.  Ruby provides a [`Struct`](https://ruby-doc.org/core-2.7.0/Struct.html) class for structured data, which gives you these accessors methods automatically

This is a contrived example, but instead of this:

```ruby
class MyRect
  attr_reader :rect_dims

  def initialize(arr)
    @rect_dims = arr
  end
  def show_size
    puts "Size: #{rect_dims[0]}x#{rect_dims[1]}"
  end
end
```

Do this:

```ruby
class MyRect
  attr_reader :rect_dims
  def initialize(arr)
    @rect_dims = make_dim_struct(arr)
  end
  def show_size
    puts "Size: #{rect_dims.width}x#{rect_dims.height}"
  end

  RectDims = Struct.new(:width, :height)
  def make_dim_struct(arr)
    RectDims.new(arr[0], arr[1])
  end
end
```

As before, the whole point is to localize the definition of your data's structure to one single place, should it need to change again.

## rubocop

[Rubocop](https://github.com/rubocop-hq/rubocop) is a linter that adheres to the [Ruby Style Guide](https://rubystyle.guide/).  Do this:

```ruby
$ gem install rubocop
```

Then always do this:

```ruby
$ rubocop myFile.rb
```

Fix everything it says to fix, and if you don't understand what it's saying or why, look it up.  Learning!

Here's a [repl.it link](https://repl.it) with the code from this post.

## To be continued...

There's actually a bunch of cool stuff out there.  Methods like [`gsub`](https://apidock.com/ruby/String/gsub) and [`inject`](https://apidock.com/ruby/Enumerable/inject) stand out as some pleasant surprises from my first explorations.  This comment from [`@ben`](https://dev.to/ben/) has been spot on:

> Ruby is a scripting language at its core. One command after another. Almost brutally simplistic.  I’d recommend opening up IRB and typing some commands. It’s actually pretty close to coding in the environment.  Ruby is sort of object oriented, sort of functional, and has a few ways to do the same thing most of the time. It’s a bit of a free-for-all.

There really is a bit of everything here.  It's got a bit of pretty much every language I've previously used!  Nailing down the clearest solution or most correctly applied pattern is a little tricker.

I looked at Ruby a long time ago, as one of my very first forays into programming.  I liked how concise it was, and [why's poignant guide](https://poignant.guide/) is a really fun read - if you haven't read it, you should at least give it a try even if you're not necessarily targetting Ruby just for how unique it is.

However, I'm having a *lot* more fun with the language returning again as a more experienced coder.  I think having the context I've built using a variety of *different types* of languages has helped me understand how to apply Ruby effectively.  It was difficult for me to learn how to use Ruby at the same time as learning how to program in a general sense, despite how easy it is to get up and running.

I don't know that I'd personally recommend Ruby as a first programming language.  There are so many paradigms available with so little friction.  Do you agree or disagree?

*Photo by Road Trip with Raj on Unsplash*