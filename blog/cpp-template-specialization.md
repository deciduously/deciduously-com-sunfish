---
title: C++ Template Specialization - Syntax Note
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--UE24ZL6H--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/r405j8k2c1z9pm0lmrl7.jpg
tags: beginners, cpp, devjournal, todayilearned
published: true
edited: 2020-01-24T12:00:00.000Z
---
I sat down yesterday with [@codemouse92](https://dev.to/codemouse92/) via [Visual Studio Live Share](https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare) for [VS Code](https://code.visualstudio.com/) -- which is an *awesome* tool - to do a mostly straightforward re-arrangement of some C++.  Unexpectedly, we ran into something, well...unexpected.  To DEV!

## The Concept

In C++, you can write a template:

```cpp
template <typename T>
T myIdentityFunction(T val)
{
   return val;
}
```

This nearly useless function just returns whatever is passed in, no matter the type.  You use it with a concrete type, like this:

```cpp
#include <iostream>

int main()
{
    int someInt = 5;
    int aCopyOfTheSameInt = myIdentityFunction(someInt);
    std::cout << aCopyOfTheSameInt << "\n";
}
```

This will output 5, as expected:

```cpp
$ clang++ test.cpp -o test
$ ./test
5
```

When it gets used, the compiler will generate the specialized version and insert that in your binary:

```cpp
int myIdentityFunction(int val)
{
    return val;
}
```

As I just learned today, you can [specialize](https://en.cppreference.com/w/cpp/language/template_specialization) what types end up in your templates to retain control over what the compiler will guess:

```cpp
template int myIdentityFunction(int val);
```

This is a silly example, but this ability lets you do things like partially specialize a `template<typename T, typename U>` to `template<int, typename U>`, and also makes implicit behaviour explicit, giving you back the keys from the compiler.  You just define one for each type you need.

## The Context

I don't actually know if the fact we're doing this in headers is relevant or not, but I'm including it for completeness in case somebody *does* know and wants to elaborate on this.  I think the issue is just "in a class" vs. "not in a class".

We were refactoring a library to be [header-only](https://en.wikipedia.org/wiki/Header-only).  In a standard library, you have your declarations in `someModule.hpp`:

```cpp
class idksomefunctions
{
public:
    idksomefunctions() = delete; // specify there should be no constructor

    template <typename T>
    static T myIdentityFunction(T);  // Template declaration
};
```

And then a corresponding `someModule.cpp` with the actual implementations and specializations:

```cpp
#include "someModule.hpp"

template <typename T>
T idksomefunctions::myIdentityFunction(T val)
{
    return val;
}

// Any specializations live here
template int idksomefunctions::myIdentityFunction(int val);
```

To refactor this into a header, you just combine them both in `someModule.hpp`:

```cpp
class idksomefunctions
{
public:
    idksomefunctions() = delete; // specify there should be no constructor

    template <typename T>
    static T myIdentityFunction(T val)
    {
        return val;
    }

    template static int myIdentityFunction(int val);  // right??
};
```

Not quite:

```
$ clang++ test.cpp -o test
In file included from test.cpp:5:
./test.hpp:15:14: error: expected '<' after 'template'
    template static int myIdentityFunction(int val);  // right??
             ^
1 error generated.
```

Okay, try the other syntax:

```diff
-  template static int myIdentityFunction(int val);
+  template <> static int myIdentityFunction(int val);
```

Good to go!

## The Switcheroo

Now, `idksomefunctions` doesn't really need to be a class - it's just, I don't know, some functions.  This could just be a [namespace](https://en.cppreference.com/w/cpp/language/namespace).  No more constructor thing, no more [`static`](https://en.cppreference.com/w/cpp/language/static) or `public` (or `storage class` errors), just some good ol' functions:

```diff
- class idksomefunctions
+ namespace idksomefunctions
  {
-  public:
-     idksomefunctions() = delete; // specify there should be no constructor

      template <typename T>
-     static int myIdentityFunction(T val)
+     T myIdentityFunction(T val)
      {
          return val;
      }

      template <>
-     static int myIdentityFunction(int val);
+     int myIdentityFunction(int val);
  }
```

Great!  But wait:

```
$ clang++ test.cpp -o test
/bin/x86_64-unknown-linux-gnu-ld: /tmp/test-d48bb0.o: in function `main':
test.cpp:(.text+0x13): undefined reference to `int idksomefunctions::myIdentityFunction<int>(int)'
clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
```

That's no good.  There's one more change to make:

```diff
      template <typename T>
      int myIdentityFunction(int val)
      {
          return val;
      }

-     template <>
+     template
      int myIdentityFunction(int val);
```

Gotta take out the `<>` thingy, back to where we started!  Now it'll compile:

```cpp

#include <iostream>

namespace idksomefunctions
{
    template <typename T>
    int myIdentityFunction(T val)
    {
        return val;
    }

    template int myIdentityFunction(int val);
};

int main()
{
    int someInt = 5;
    int aCopyOfTheSameInt = idksomefunctions::myIdentityFunction(someInt);
    std::cout << aCopyOfTheSameInt << "\n";
}
```

## The Recap

Inside a class, you specialize via:

```cpp
template<> static int myIdentityFunction(int val);
```

Outside of a class, though, you omit the thingy:

```cpp
template int myIdentityFunction(int val);
```

## The Question

What am I saying when I say `template <>` versus `template` here?

*Photo by Ricardo Gomez Angel on Unsplash*