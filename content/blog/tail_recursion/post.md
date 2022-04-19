---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--aId0glXB--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/zrrpq5uedlhiguf9haid.jpg
date: May 22, 2019
title: Tail Recursion
tags:
  - beginners
  - functional
  - cpp
---

I'm a (wannabe) functional programming zealot, and you recur all over the place when you're programming functionally. It's often via library functions like `map` and `reduce` as opposed to writing your own recursive functions, but it's a super common theme. It's so satisfying to get right, and leads to some wonderfully concise, elegant implementations.

But _gosh_ can it be slow. It's fine for small cases but can seriously bottleneck larger programs and inputs, and not always in ways that are easy to predict.

One way to alleviate this pain is called _tail recursion_. This means that you can recur, but you must do it only in the tail position of the function call which means the recursive call the last thing called as the return value. C++ has a highly optimizing compiler that can actually optimize away the recursion in this case, making tail recursive functions more performant than non-tail recursive ones.

The basic strategy for this is to reuse the stack frame. Basically, every time a function is called, it pushes a new _frame_ onto the _call stack_. This frame contains state information for the evaluation of this function (more accurately, subroutine), such as the parameters it was called with. This frame had to be allocated somewhere in memory and populated and then pushed onto this stack. All of that took time and resources, especially if the values themselves are large. In a recursive function you're asking for this to happen repeatedly, often with larger and larger parameters. It can get nuts, especially because these frames are only popped off the call stack and de-allocated when the subroutine completed - which will be _after_ all its children are done.

If you recur in tail position, though, the stack frame actually doesn't need to change for the next recursive call. Instead, the values can just get swapped in place, and the stack frame that's already been allocated for THIS call is just recycled. No pushing more and more frames on top of the call stack, allocating more and more memory for more and more temporary function calls. It all just happens in place in memory. Way faster!

C++ compilers are often even smarter than that, though, and might rip our your recursion and pop a regular loop in its place, which will be even faster yet.

I'm gonna keep the examples super simple. Here's how you might define `factorial` in a recursive manner in C++:

```cpp
int factorial(int n) {
    if (n > 1) {
        return n * factorial(n - 1);
    }
    else {
        return 1;
    }
}
```

This implementation, while nice and neat and easy on the eyes, is not tail recursive - it calls factorial inside of itself _and then_ multiplies that result by n. In this case, multiplication is in the tail position, not the recursive call. To get it to be tail recursive, that multiplication needs to happen inside the parameter list of the function call (or in some other manner before it), and to do that you can supply a default value:

```cpp
int factorial(int n, int b = 1) {
	if (n == 0) {
		return b;
        }
	return factorial(n - 1, b * n);
}
```

This function works in almost the same way, just reorganized so that the recursive call is in tail position and the multiplication is inside the call. Because of operator precedence (and how this function works), the multiplication is evaluated first. We're storing the result as we recur down to 0 in this phantom b parameter. It's kind of like carrying extra state. The first iteration our default of 1 is multiplied by the n value supplied. If the supplied n was zero, we just return that one, and otherwise when eventually we have decremented n to zero, b will hold the value we want.

Often the key with these is to see if you can fit your base case(s) into your parameters, or use an auxiliary function that actually recurs with all the extra information stored in its parameters. Another common recursive function example is the Fibonacci series:

```cpp
int fibonacci(int n) {
    if (n == 0) {
		return 0;
	}
	else if (n == 1) {
		return 1;
	}
	else {
		return fibonacci(n - 1) + fibonacci(n - 2);
	}
}
```

Should do the trick, non?

**_NON_**

This will hose you so fast it's not even funny. Toss it in a loop and watch it slow to an absolute crawl before your very eyes:

```cpp
int main() {
	int n;
	std::cout << "nth fibonacci" << std::endl << "N: ";
	std::cin >> n;
	for (int i = 0; i <= n; i++) {
		std::cout << fibonacci(i) << " ";
	}

	return 0;
}
```

Try `n = 200`, I dare you.

Luckily, we can just refactor in those default base cases to make it tail recursive:

```cpp
int fibonacci(int n, int a = 0, int b = 1)
{
	if (n == 0)
		return a;
	if (n == 1)
		return b;
	return fibonacci(n - 1, b, a + b);
}

```

In this case the series is built on two base values, 0 and 1. No matter, we can pop 'em both in the parameters and start counting up from there. We just hop along the line by shifting the `b` parameter to `a` and building a new `b`.

Pop `200` in to your loop printer. They'll all come popping out immediately, integer overflow issues and all. Hey, it's a hell of a lot better than getting bored and copping out after after dozen iterations, right?

Your move in the comments. Let me see you shake those tail-recursive functions, or optimize these further!
