---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--Av_bUH0n--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/jhwnn4ryog0mjms4is8o.jpg
edited: 2019-10-03T12:00:00.000Z
title: Overly Functional C++: The BenFolds Five
published: true
description: Some common operations implemented in terms of a fold.
tags: beginners, cpp, functional, devjournal
---
# Two-Part Series, Chapter 3

What a plot twist!  I made almost no effort to avoid [the pun](https://en.wikipedia.org/wiki/Ben_Folds_Five) and I'm only a little sorry.

Despite being a trilogy now, this post stands alone if you're familiar with the concept of the `fold` or `reduce` [higher-order function](https://en.wikipedia.org/wiki/Fold_(higher-order_function)).

In [part 2](https://dev.to/deciduously/overly-functional-c-the-fold-4kid) of yesterday's minddump, I documented my first stab at a specific fold in C++.  I had gotten stuck, though, in trying to make it generic.

 I've [said it before](https://dev.to/deciduously/you-lot-are-great-cea) and I'll say it again: ask DEV stuff, they know things.  Thanks to [@curtisfenner](https://dev.to/curtisfenner) and [@markboer](https://dev.to/markboer) I was able to write the intended generic `fold` function I had set out to write initially with almost no modification.  This newly-generic `fold` function is a building block, and if you give a nerd a `fold`...

## Five Library Functions

`BenFolds` is a class with five static member functions.  It defines `BenFolds::fold()` as a generic version of the code from part 2, and then defines `or`, `any`, `elem`, and `map` in terms of this fold.  I'll walk through each, or you can grab the [gist](https://gist.github.com/deciduously/fdb8ee23b4f3d73e4340ef85359edce6).  This gist compiled and executed successfully for me using `g++ -std=c++11 -o benfolds benfolds.cpp` on g++ 9.2.0.

### Fold

This is the only definition with any substance, the rest will all specify parameters to run through this fold:

```cpp
template <typename T, typename R, typename BinOp>
static R fold(std::vector<T> elems, R acc, BinOp func)
{
    int length = elems.size();
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
            R newAcc = func(acc, elems[0]);
            // Create a new input from the second element onward of the current input
            std::vector<T> newInput(elems.begin() + 1, elems.end());
            // Recur with rest of list and new accumulator
            return fold(newInput, newAcc, func);
        }
}
```

This implementation is identical to the solution from [part 2](https://dev.to/deciduously/overly-functional-c-the-fold-4kid) apart from the parameterized types.  The other four "backup" functions are just specific cases of this `fold`.

As [@curtisfenner](https://dev.to/curtisfenner) pointed out, this naive implementation is needlessly expensive.  The recursion is in tail position, which the compiler does optimize for, but each call is allocating a brand new vector to swap into the stack frame.  If you needed to optimize this further, you could consider instead passing ever-smaller subslice references to the same vector, or even mutating the original vector in place instead.

### foldOr

The simplest application just takes a list of booleans and tells you if any of them are `true`.  The `T` in `BenFolds::fold()` is `bool`:

```cpp
static bool foldOr(std::vector<bool> bools)
{
    return fold<bool>(bools, false, [](bool acc, bool curr) { return acc || curr; });
}
```

The initial accumulator is `false`.  We call `||` on this accumulator against each element of the passed collection.  At the end, if any element was `true`, the accumulator flipped to `true` and stuck.  Otherwise, nothing was found it's still `false`.

To test, I added a quick throwaway to `main()`:

```cpp
vector<bool> bools = {false, false, false, true, false};
cout << "Testing foldOr...\nThis should be true: " << bf.foldOr(bools) << "\n";
```

### foldAny

The `any` function is just a generalization of `or`:

```cpp
template <typename T, typename Predicate>
static bool foldAny(std::vector<T> elems, Predicate p)
{
    return fold(elems, false, [p](bool acc, T curr) { return acc || p(curr); });
}
```

It's really almost the same thing.  We're still calling `||` on the accumulator and this element, but we're passing the element through some other predicate `p` before checking for truth.

For example, see if the inputs has any even numbers (it should) or anything greater than 10 (it shouldn't):

```cpp
cout << "Testing foldAny...\nAre there even elements in the set: " << bf.foldAny(nums, [](int n) { return n % 2 == 0; }) << "\n";
cout << "Are there elements greater than 10: " << bf.foldAny(nums, [](int n) { return n > 10; }) << "\n";
```

### foldElem

The `elem` function checks if an element exists in a collection, which is a specialization of `any` so we can reuse that definition:

```cpp
template <typename T>
static bool foldElem(std::vector<T> elems, T elem)
{
    return foldAny(elems, [elem](T curr) { return elem == curr; });
}
```

It calls `foldAny` defining the predicate as equality against a specific element.  We can test by checking for a specific number, no need to pass a lambda:

```cpp
cout << "Testing foldElem...\nIs the number 2 present in the set: " << bf.foldElem(nums, 2) << "\n";
cout << "Is the number 8 present in the set: " << bf.foldElem(nums, 8) << "\n";
```

### foldMap

We can also define a `map` function from this `fold` that builds the target vector in the accumulator:

```cpp
template <typename T, typename Op>
static std::vector<T> foldMap(std::vector<T> elems, Op func)
{
    return fold(elems, std::vector<T>(), [func](std::vector<T> acc, T curr) {
        acc.push_back(func(curr));
        return acc;
    });
}
```

I tested this one by doubling each element in the input:

```cpp
cout << "Testing foldMap...\nHere's each element doubled: " << bf.foldMap(nums, [](int elem) { return elem * 2; }) << "\n";
```

Running through all the tests as defined yields this output:

```
Set: [0, 1, 2, 3, 4]
Testing fold...
Sum: 10
Testing foldOr...
This should be true: 1
Testing foldAny...
Are there even elements in the set: 1
Are there elements greater than 10: 0
Testing foldElem...
Is the number 2 present in the set: 1
Is the number 8 present in the set: 0
Testing foldMap...
Here's each element doubled: [0, 2, 4, 6, 8]
```

![Good enough](https://media1.tenor.com/images/39f958c6a71049618e89d6bbfc8e96a2/tenor.gif)

*Photo by Oscar Keys on Unsplash*
