---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--vk5AiHBF--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/zdhqlnof5w8m2eivc21l.jpg
date: 2019-10-19T12:00:00.000Z
title: "Overly Functional C++: The Fold"
tags:
  - cpp
  - beginners
  - devjournal
  - help
---

# C++ For Hipsters

This is part two of a three-part post, but don't worry - each part stands alone. You don't need to read any of the first post before reading this.

## The Goal

This code implements a the higher-order "fold" function for a `std::vector<int>`, which is a variable-size one-dimensional collection of integers available in the standard library. The actual `int`-type is system and compiler dependent, but for this toy demo that's not a concern of mine. For trivia, mine are 32-bit `long`s.

I did this to see how hard it would be. The answer turned out to be "not".

The code that appears here compiles as shown using a command like `g++ --std=c++11 -o foldtest foldtest.cpp`, and should work without modification on other compilers that support C++11.

## The Humble Fold

In functional programming, the [fold function](https://dev.to/deciduously/know-when-to-fold-em-1466) is a commonly-used building block. It abstracts away the recursive part of the function definition for a very common use case - producing an accumulated result by processing each element in a collection - into a flexible, safe API.

Most widely-used mainstream languages these days like JavaScript, Java, and C++ all actually allow this sort of thing, but _most_ (not all) code I see in the wild tends toward an imperative pattern. The classic `for` loop, wherein you increment a counter to use to index into an array, or its more snazzy cousin the iterator-backed `for item in collection`, become second nature quickly for many people learning how to code. It quite often can be the better option, too, depending on your needs for the code.

### The expected output

When implemented correctly, we should expect the following output from our test code:

```
Nums: [1, 2, 3, 4, 5]
Accumulator: 0
Summing...
Result: 15
```

### The implementation

The `main()` function to generate that output looks like this:

```cpp
#include <functional>
#include <iostream>
#include <vector>

// ..

int main()
{
    using std::cin;
    using std::cout;
    using std::vector;
    vector<int> nums = {1, 2, 3, 4, 5};
    int acc = 0;
    cout << "Nums: " << nums << "\nAccumulator: " << acc << "\nSumming...\n";
    int result = fold(nums, acc, [](int acc, int curr) { return acc + curr; });
    cout << "Result: " << result;
    return 0;
}
```

We define a hardcoded vector `[1,2,3,4,5]`, display it to the user, and then pass it to our `fold` function with an accumulator and the lambda to use, displaying that result.

Before implementing the fold itself, I defined a quick template to pretty-print a vector like shown in the expected output, for debugging purposes:

```cpp
// Pretty-print a vector
template <typename T>
std::ostream &operator<<(std::ostream &stream, const std::vector<T> vec)
{
    stream << "[";
    for (int i = 0; i < vec.size(); i++)
    {
        stream << vec[i];
        if (i < vec.size() - 1)
        {
            stream << ", ";
        }
    }
    stream << "]";
    return stream;
}
```

All that's missing is the actual `fold`:

```cpp
// Fold a binary operation over a vector of ints
int fold(std::vector<int> nums, int acc, std::function<int(int, int)> func)
{
    int length = nums.size();
    if (length == 0)
    {
        // base case - empty list
        // Sub-problem is trivial - we're already done.  The passed accumulator holds the result
        return acc;
    }
    else
    {
        // Calculation is not complete - sub-problem is also trivial
        // Call function on accumulator using first element in vector as operand
        int newAcc = func(acc, nums[0]);
        // Create a new input from the second element onward of the current input
        std::vector<int> newInput(nums.begin() + 1, nums.end());
        // Recur with rest of list and new accumulator
        return fold(newInput, newAcc, func);
    }
}
```

The body is simple. The base case simply returns, and the recursive case builds the new adjusted inputs and passes them back to the function. The one part I had to look up was [`std::function`](https://en.cppreference.com/w/cpp/utility/functional/function), used for the type of the lambda parameter. I had written essentially what I ended up with in pseudocode, not knowing how I'd actually need to implement the working version, only to find the exact construct I wanted actually existed. Thanks, C++.

To verify that this code performs reasonably, I replaced the hardcoded input with a loop to populate a vector with numbers from 0 through n:

```cpp
vector<int> nums;
for (int i = 0; i < 43642; i++)
{
    nums.push_back(i);
}
```

On my computer, this code sums this vector in just over two seconds:

```
$ time ./foldsum
Summing...
Result: 952290261
real    0m2.171s
user    0m0.405s
sys     0m1.763s

```

However, any higher value dumps core.

## Call For Help

One of those hashtags is not like the others... I also have a question about extending this code. This is an extremely limited fold function, with the type of the collection to fold over fully specified as `std::vector<int>`. This sounds like a job for a template to me, but from what I understand C++11 doesn't fully support templated lambdas yet. Is that more or less accurate?

_Photo by Kelly Sikkema on Unsplash_
