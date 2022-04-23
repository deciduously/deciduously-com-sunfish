---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--9GzgvYXH--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/do9pmhc297wl6x8pjlt0.jpg
date: 2019-11-24T12:00:00.000Z
title: Validate a Credit Card Number with Functional JavaScript
description: A walkthrough of a Luhn algorithm in JavaScript using a functional style
tags:
  - beginners
  - functional
  - javascript
  - tutorial
---

## Dolla Dolla Bill, Y'all

Credit card companies are responsible for a high volume of highly sensitive global network traffic per minute with no margin for error. These companies need to ensure they are not wasting resources processing unnecessary requests. When a credit card is run, the processor has to look up the account to ensure it exists, then query the balance to ensure the amount requested is available. While an individual transaction is cheap and small, the scales involved are enormous.
There were [39.2 million transactions per day](https://www.statista.com/statistics/719708/card-payments-per-day-forecast-united-kingdom/) in the UK alone in 2016. The linked analysis projects 60 million for that region by 2026. Clearly, anything that can reduce load is necessary to explore.

This is a beginner-level post. Some familiarity with JavaScript is assumed but not necessarily functional programming.

## What's In A Number

At a glance, a credit card number just appears to be a sequence of digits. You may have noticed that the major processing providers have their own prefixes. Visa cards all start with a 4, MasterCard with 5, Discover with 6, and American Express are 3 (and 15 digits instead of 16). Further, financial institutions will have their own 4-6 digit prefixes. People who work at point of sale systems or are otherwise involved with financial processing will notice these patterns quickly. For example, Discover credit cards start with 6011, a 4117 will be a Bank of America debit card, and 5417 is Chase Bank. This is known as the BIN, or Bank Identification Number. There's a [large list here](https://www.bindb.com/bin-list.html).

However, this is all a network routing concern, and still adds to the network's load to resolve. To try to ensure all lookup requests actually correspond to real accounts, all numbers have a **checksum** built in, which is a means of detecting errors in data. A credit card number consists of your card provider's BIN attached to your individual account number, but the final digit is a checksum digit which can be used to validate for errors without ever querying a server.

### Protip

"I'm a BIN and routing number encyclopedia" is a **terrible** party icebreaker. If you've really gotta flex this side of you, ease in with zipcodes or something first. Read the room.

### Luhn algorithm

The specific type of checksum is called the [Luhn formula](https://en.wikipedia.org/wiki/Luhn_algorithm), [US Patent 2,950,048](https://patents.google.com/patent/US2950048) (but public domain since 1977). To validate a number via the Luhn algorithm, you add a check digit. Then, after performing the formula on the original number, you see if this check digit corresponds to your result.

1. Split the full number into individual digits.

1. Start with the rightmost _excluding_ the check digit and double every second, moving left.

1. If any of those doubled digits ended up greater than 9, add the digits together (or subtract 9, if that's your jam).

1. Take the sum of all the digits and the check digit.

1. If the total modulo 10 equals 0, the number is valid.

For an example, the number `4012-8888-8888-1881` is a valid Visa-formatted account number, used for testing. You can't charge it, but it should validate with this algorithm.

1. Split into digits: `4 0 1 2 8 8 8 8 8 8 8 8 1 8 8 1`.

1. Double every second except the check digit, right to left: `8 0 2 2 16 8 16 8 16 8 16 8 2 8 16 1`.

1. Add digits of any above nine: `8 0 2 2 7 8 7 8 7 8 7 8 2 8 7 1`.

1. Sum the digits: `90`.

1. Is it a multiple of 10? Yep!

This number checks out, it could possibly be a valid Visa card so we're clear to make the network request.

## Implement

To follow along, you'll need [Node](https://nodejs.org/en/). I'm using [pnpm](https://github.com/pnpm/pnpm), feel free to use `npm` or `yarn` instead. Create a new project:

```txt
$ mkdir luhn
$ cd luhn
$ pnpm init
// follow prompts
$ touch index.js
```

Throw a stub into `index.js` to get hooked up:

```js
const luhn = {};

luhn.validate = (numString) => {
  return false;
};

module.exports = luhn;
```

### Unit tests

Before hopping into the implementation, it's a good idea to have some unit tests ready to go. Add `mocha`:

```txt
$ pnpm install mocha
$ mkdir test
$ touch test/test.js
```

In `package.json`, set the `test` script to run [`mocha`](https://mochajs.org/):

```json
"scripts": {
  "test": "mocha"
},
```

Now add the following tests to `test/test.js`:

```js
const assert = require("assert").strict;
const luhn = require("../index.js");

describe("luhn", function () {
  describe("#validate()", function () {
    it("should accept valid Visa test number", function () {
      assert.ok(luhn.validate("4012-8888-8888-1881"));
    });
    it("should accept valid MasterCard test number", function () {
      assert.ok(luhn.validate("5105-1051-0510-5100"));
    });
    it("should accept valid Amex test number", function () {
      assert.ok(luhn.validate("3714-496353-98431"));
    });
    it("should reject invalid numbers", function () {
      assert.equal(luhn.validate("1234-5678-9101-2131"), false);
    });
  });
});
```

Don't worry, those aren't real accounts, just some valid test numbers from [here](https://www.paypalobjects.com/en_GB/vhelp/paypalmanager_help/credit_card_numbers.htm).

As expected, running `npm test` should confirm that our stub has some work to do:

```txt
Luhn
  #validate()
    1) should accept valid Visa test number
    2) should accept valid MasterCard test number
    3) should accept valid Amex test number
    ✓ should reject invalid numbers
```

I'm sticking to a functional style for this implementation, wherein instead of mutating state and looping we'll get to the final result by defining transformations over data.

### Split Digits

The first order of business is to get the digits out of the string we're passed. We can just discard anything that isn't a number using [`String.prototype.replace()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace).

```js
const to_digits = (numString) =>
  numString
    .replace(/[^0-9]/g, "")
    .split("")
    .map(Number);
```

The regular expression uses `^` to match anything that _isn't_ a digit from 0-9. The trailing `g` indicates we want to match globally and replace all matches found with nothing (removing it from the string). If omitted, only the first match is replaced and the remaining string is untouched. Then, we split into individual characters, one per digit, and convert them all from characters to numeric values.

### Set The Stage

Back in `luhn.validate()`, let's store our digit array using this function and hold on to the check digit for later:

```diff
luhn.validate = numString => {
+ const digits = to_digits(numString);
+ const len = digits.length;
+ const luhn_digit = digits[len - 1];
+ const total = 0; // TODO
  return false;
};
```

To get to our final validation, we're going to perform a series of transformations on this digit array to reduce it to a final total. A valid number will produce a result that's a multiple of 10:

```diff
luhn.validate = numString => {
  const digits = to_digits(numString);
  const len = digits.length;
  const luhn_digit = digits[len - 1];
  const total = 0; // TODO
- return false;
+ return total % 10 === 0;
};
```

### Get The Total

We already talked this through in English. Let's take a stab in pseudocode:

```js
const total = digits
  .doubleEveryOtherFromRightMinusCheckDigit()
  .map(reduceMultiDigitVals)
  .addAllDigits();
```

We've got to do that doubling step on the correct numbers in the account number, then transform anything that ended up with multiple digits, then get the total of everything together.

For this step, we can use [`Array.prototype.slice()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice) to get a subset of the digits array that has everything except for the check digit. Going right-to-left can be achieved with [`Array.prototype.reverse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse):

```diff
const total = digits
- .doubleveryOtherFromRightMinusCheckDigit()
+ .slice(0, -1)
+ .reverse()
+ .map(doubleEveryOther)
  .map(reduceMultiDigitVals)
  .addAllDigits();
```

The [`Array.prototype.map()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map) calls can just be left as-is, we can define the functions we need in a moment. The final step, adding everything together, can be handled with [`Array.prototype.reduce()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce). This method produces a single result from a collection by calling a function on each element and an accumulator. By adding each element to a running total, we can produce a sum. Instead of starting from 0, though, we can just start from the checksum digit we had stored earlier:

```diff
const total = digits
  .slice(0, -1)
  .reverse()
  .map(doubleEveryOther)
  .map(reduceMultiDigitVals)
- .addAllDigits()
+ .reduce((current, accumulator) => current + accumulator, luhn_digit);
```

Good to go!

#### Define Transformations

We've got two operations left undefined in the above pipeline, `doubleEveryOther` and `reduceMultiDigitVals`. In both, we're going through each digit and conditionally adjusting the value there. It's either every other digit, or if a digit is greater than a certain threshold, but in both cases the basic mapping function takes the same format - it conditionally transforms:

```js
const condTransform = (predicate, value, fn) => {
  if (predicate) {
    return fn(value);
  } else {
    return value;
  }
};
```

This works somewhat like the ternary operator but as a function. Each instance of this is just a specified case of a conditional transform:

```js
const doubleEveryOther = (current, idx) =>
  condTransform(idx % 2 === 0, current, (x) => x * 2);

const reduceMultiDigitVals = (current) =>
  condTransform(current > 9, current, (x) => x - 9);
```

Both of these accept argument lists that are compatible with `map()`, so can be plugged in directly as-is. One includes the current element's index and one doesn't, and both just pass through to this helper transform. If the predicate is satisfied the element will be transformed per the final transforming function, and otherwise it's left untouched.

## Wrapping Up

Putting it all together:

```js
const to_digits = (numString) =>
  numString
    .replace(/[^0-9]/g, "")
    .split("")
    .map(Number);

const condTransform = (predicate, value, fn) => {
  if (predicate) {
    return fn(value);
  } else {
    return value;
  }
};

const doubleEveryOther = (current, idx) =>
  condTransform(idx % 2 === 0, current, (x) => x * 2);

const reduceMultiDigitVals = (current) =>
  condTransform(current > 9, current, (x) => x - 9);

const luhn = {};

luhn.validate = (numString) => {
  const digits = to_digits(numString);
  const len = digits.length;
  const luhn_digit = digits[len - 1];

  const total = digits
    .slice(0, -1)
    .reverse()
    .map(doubleEveryOther)
    .map(reduceMultiDigitVals)
    .reduce((current, accumulator) => current + accumulator, luhn_digit);

  return total % 10 === 0;
};

module.exports = luhn;
```

Check it out with `pnpm test`:

```txt
  luhn
    #validate()
      ✓ should accept valid Visa test number
      ✓ should accept valid MasterCard test number
      ✓ should accept valid Amex test number
      ✓ should reject invalid numbers


  4 passing (3ms)
```

This algorithm is used for a variety of different types of data verification, not just credit card numbers. Maybe you could integrate it into your next project's design! Adding a checksum to your DB keys can help protect against data transmission errors, and very simple verification like this is easy to get started with.

### Challenge

Extend this code to provide a method that can add a correct Luhn checksum to any arbitrary number. The check digit will be the number you need to add to your total to get to multiple of 10.

_Photo by Clay Banks on Unsplash_
