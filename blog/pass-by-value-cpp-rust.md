---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--AJ9eCaT8--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/uoal1zm230b42dzo58ca.jpg
edited: 2019-09-17T12:00:00.000Z
title: Pass-By-Value in C++ and Rust
published: true
description: A comparison of Rust and C++ pass-by-value semantics
tags: beginners, cpp, rust
---
C++ and Rust are often compared to each other.  They occupy a similar space in terms of power and flexibility - neither has a garbage collector and thus can fit in resource-constrained domains, yet both provide richer high-level tools than a language like C which increase safety and correctness.

However, the experience of writing a program in each can be pretty different.  Once such difference beginners in Rust will run into quickly is what happens when you pass a parameter by value.  Rust handles this situation differently than C++, and it's worth exploring why.

## C++

In C++, passing by value passes a **copy** of the object into the function.  This is fine for primitives like integers - my 5 is the same as your 5.  The fact that they're distinct values in memory won't ever matter for their use, because the meaning of 5 isn't context or state dependent.  Lots of other things are, though.  When an object is copied in C++, its *copy constructor* gets called.  These have a prototype that looks like this:

```cpp
classname (const classname &obj);
```

When an object is passed as a parameter to a method, this constructor is used to copy the object into the function body.  Check out that keyword at the beginning of the parameter list, "const".  This means we can't use this constructor to make any changes to the initial object.  Instead, it's just going to create a new copy, which is what's getting used inside any function. To illustrate, here's a simple class with just a single data member, a default constructor, and a getter and setter:

```cpp
class CoolObject
{
    int coolValue;

public:
    CoolObject()
    {
        coolValue = 5;
    }
    int getCoolValue() const
    {
        return coolValue;
    }
    void setCoolValue(int val)
    {
        coolValue = val;
    }
};
```

We'll write a function that takes one of these objects by value and sets it to 10:

```cpp
#include <iostream>

void setCoolValueToTen(CoolObject co)
{
    using std::cout;
    cout << "Current: " << co.getCoolValue() << " | Setting...\n";
    co.setCoolValue(10);
    cout << "New: " << co.getCoolValue() << "\n";
};
```

If we make two of these, and use this function on one, you'd expect it to stick, right?

```cpp
int main()
{
    using std::cout;
    CoolObject co1;
    CoolObject co2;
    cout << "co1: " << co1.getCoolValue() << " | co2: " << co2.getCoolValue() << "\n";
    setCoolValueToTen(co2);
    cout << "co1: " << co1.getCoolValue() << " | co2: " << co2.getCoolValue();
    return 0;
}
```

Instead, we get the following:

```
co1: 5 | co2: 5
Current: 5 | Setting...
New: 10
co1: 5 | co2: 5
```

The code inside the setCoolValueToTen() function is operating on its very own copy, made from and identical to co2 when it was passed in but entirely distinct from it.  Calling the setter on this local instance has no effect on co2, because it's no longer involved.

If you pass by value, all your changes are stuck in this new local copy and never make it back to your intended target.  A reference to the original solves this problem:

```cpp
void reallySetCoolValueToTen(CoolObject &co) // Just take a reference - rest is identical!
{
    using std::cout;
    cout << "Current: " << co.getCoolValue() << " | Setting...\n";
    co.setCoolValue(10);
    cout << "New: " << co.getCoolValue() << "\n";
}

int main()
{
    using std::cout;
    CoolObject co1;
    CoolObject co2;
    cout << "co1: " << co1.getCoolValue() << " | co2: " << co2.getCoolValue() << "\n";
    setCoolValueToTen(co2);
    cout << "co1: " << co1.getCoolValue() << " | co2: " << co2.getCoolValue() << "\n";
    reallySetCoolValueToTen(co2);
    cout << "co1: " << co1.getCoolValue() << " | co2: " << co2.getCoolValue() << "\n";
    return 0;
}
```

The second call works as expected:

```
co1: 5 | co2: 5
Current: 5 | Setting...
New: 10
co1: 5 | co2: 5
Current: 5 | Setting...
New: 10
co1: 5 | co2: 10
```

## Rust

Let's attempt to re-implement this small program in Rust.  Here's our `CoolObject`:

```rust
struct CoolObject {
    cool_value: i32,
}

impl CoolObject {
    fn get_cool_value(&self) -> i32 {
        self.cool_value
    }
    fn set_cool_value(&mut self, val: i32) {
        self.cool_value = val;
    }
}

impl Default for CoolObject {
    fn default() -> Self {
        Self { cool_value: 5 }
    }
}
```

We need a function to set the value to ten, taking the parameter by value:

```rust
fn set_cool_value_to_ten(mut co: CoolObject) {
    println!("Current: {} | Setting...", co.get_cool_value());
    co.set_cool_value(10);
    println!("New: {}", co.get_cool_value());
}
```

We're already starting to see a problem - we can't just mutate values without asking first, like we can in C++.  If I hadn't included that `mut` in the parameter list, the `set_cool_value()` call would complain: "cannot borrow `co` as mutable, as it is not declared as mutable".  We need to specifically tell the compiler that we intend to mutate the object.

Let's try to emulate the first go of the C++ version:

```rust
fn main() {
    let co1 = CoolObject::default();
    let co2 = CoolObject::default();
    println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
    set_cool_value_to_ten(co2);
    println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
}
```

Attempting to compile this code will net you an error like the following:

```
error[E0382]: borrow of moved value: `co2`
  --> src/main.rs:34:57
   |
31 |     let co2 = CoolObject::new();
   |         --- move occurs because `co2` has type `CoolObject`, which does not implement the `Copy` trait
32 |     println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
33 |     set_cool_value_to_ten(co2);
   |                           --- value moved here
34 |     println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
   |                                                         ^^^ value borrowed here after move

error: aborting due to previous error
```

And there's the problem.  When pass by value in C++, the compiler will just assume you know what you're doing and call a copy constructor for you, even if it doesn't really make sense.  If you haven't manually defined a copy constructor, no sweat - the compiler will do it's damndest to generate one for you and call that.  After all, you've passed by value, s this must be what you want!

Rust pumps the brakes.  When you pass by value, it actually **moves ownership** of the original value.  It's not copying the original object in, it's actually bringing the object from outside - but the caveat is that the calling scope no longer owns this value at all, the new function does.  When `set_cool_value_to_ten()` reaches the end of its body, this value goes out of scope!  It's dropped.  When we attempt to refer to `co2` again in the next line, we can't - it's not ours to use anymore.

In Rust, any value only has one owner.  You can borrow as many immutable references as you like, which we do when we call `get_cool_value(&self)`, or we can have one single mutable reference, like with `really_set_cool_value_to_ten(co: &mut CoolObject)`, but if there's no borrow, like with `set_cool_value_to_ten(mut co: CoolObject)`, you know ownership of this value will be moving.

This skirts the common pass-by-value bug in C++ where you think you're working with an object but you're actually just working with a copy.  C++ will just silently try to make things work, and may not be on the same page as you are.  Rust is very explicit.  It even specifically tells you that if your object did implement the `Copy` trait, it would have attempted to copy the value - but of course, this still wouldn't solve this problem.  As with C++, the solution is to refer to the original instead of move the value.  In C++, you say "take a reference", but in Rust, you'd call it a "mutable borrow":

```rust
fn really_set_cool_value_to_ten(co: &mut CoolObject) {
    println!("Current: {} | Setting...", co.get_cool_value());
    co.set_cool_value(10);
    println!("New: {}", co.get_cool_value());
}
```

We also need to declare `co2` itself as mutable:

```rust
fn main() {
    let co1 = CoolObject::default();
    let mut co2 = CoolObject::default(); // right here
    println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
    really_set_cool_value_to_ten(&mut co2); // and pass a mutable reference
    println!("co1: {} | co2: {}", co1.get_cool_value(), co2.get_cool_value());
}
```

This illustrates one of the reasons I prefer working with Rust over C++.  In C++, the programmer just has to know all of these details about how the language operates, and the compiler has no qualms about implicit actions that it takes.  You've got no help reading through your code to figure out where you've made this mistake, and even full awareness of this issue is insufficient to avoid it in 100% of cases.  Rust, on the other hand, doesn't let you ask for stupid things.  In this situation, the compiler was able to tell me in plain English why my code was incorrect and how to fix it.

*Photo by Natalia Y on Unsplash*\
