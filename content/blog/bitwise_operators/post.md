---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--yWCIQ4hc--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/4g4g4juasklhp19c0cm6.jpg
date: 2020-08-09T12:00:00.000Z
title: "Using Bitwise Operators: Why Waste Space Use Many Bits When Few Bits Do Trick?"
tags:
  - beginners
  - tutorial
  - devjournal
  - chip8
---
These days, we live in the future.  Memory is very cheap.  When we need to store an integer, in most cases it makes sense to just use an entire multi-byte value to represent it even if your value is low.  Of course, that does result in a lot of wasted space.  The number `2` as a 32-bit integer looks like this:

```txt
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
```

Just one of these bits is flipped towards the end - all the rest is just padding.

I'm working on a [Chip8](https://en.wikipedia.org/wiki/CHIP-8) emulator.  To build such an emulator, you create a software representation of the entire computer.  It has a space for the program memory, some registers for short-term storage, some counters, some boolean flags - everything it needs to execute a program.

The Chip8 is a much more limited environment than a modern computer.  The entire space for memory only comprises 4096 bytes - and some of that is reserved for the system itself.  With such a constraint it doesn't make sense to waste all this space.  Whenever possible we want to concern ourselves with a single byte at a time, so that surrounding bytes can be meaningfully applied to other parts of the program.

In fact, as we'll see, we don't even need to limit ourselves to a byte.  Using bitwise operators we can efficiently use individual bits within a byte to store and communicate information with minimal wasted space.

## The Machine

I'm doing mine in Rust, but you don't need to know Rust to follow this post.  This is what my struct representing the machine looks like:

```rust
/// The top-level software representation of the Chip8 machine
pub struct Machine {
    opcode: Opcode,
    /// Available memory space - 4K
    memory: [u8; 4096],
    /// CPU Registers
    registers: [u8; 16],
    /// Index register
    pub idx: u16,
    /// Program counter
    pub pc: u16,
    /// Graphics system - 2048 total pixels, arranged 64x32, each containing 0 or 1
    pub screen: [u8; 64 * 32],
    /// Delay timer - 60Hz, counts down if above 0
    pub delay_timer: u8,
    /// Sound timer - buzzes at 0.  60Hz, counts down if above 0
    pub sound_timer: u8,
    /// Call stack
    pub stack: [usize; 16],
    /// Stack pointer
    pub sp: usize,
    /// Keep track of the keypad - 0x0-0xF
    keys: [bool; 16],
    /// Flag to signal the screen state has changed and must be redrawn
    pub draw_flag: bool,
}
```

The biggest thing you need to know is that a `u8` is an unsigned 8-bit value, or a single byte, and a `u16` is an unsigned 16-bit value.  The `usize` types are machine-dependent pointer-sized values, used to index into arrays.  In my case they're analogous to `u64`.

There is a software representation for each part of the computer a program will interact with.  Then, you can programmatically hook up actual inputs and outputs however you like, and the running program is none the wiser.  Inside this struct everything works exactly as it would on a physical version of this machine.

## Hexadecimal Representation

When working with individual bytes hexadecimal is much more convenient than decimal.  A single hexadecimal digit maps neatly to a 4-bit value, or a *nibble*, and thus can express every possible configuration of 4 bits:

```txt
0x0 -> 0 0 0 0
0x1 -> 0 0 0 1
0x2 -> 0 0 1 0
0x3 -> 0 0 1 1
0x4 -> 0 1 0 0
0x5 -> 0 1 0 1
0x6 -> 0 1 1 0
0x7 -> 0 1 1 1
0x8 -> 1 0 0 0
0x9 -> 1 0 0 1
0xA -> 1 0 1 0
0xB -> 1 0 1 1
0xC -> 1 1 0 0
0xD -> 1 1 0 1
0xE -> 1 1 1 0
0xF -> 1 1 1 1
```

It's clear to see the pattern.  This makes hexadecimal a natural way to talk about memory in such small quantities.  A byte is 8 bits, which is two nibbles next to each other.  Using two hex digits we can represent every possible byte from `0x00`, which is all 8 zeros, to `0xFF`, which is all 8 ones.

## Opcodes

To interact with the machine state, a program consists of a series of opcodes.  Each cycle, the machine reads the current opcode and then updates the state in memory and registers according to which specific instruction is found.  When the program loads these opcodes are copied into the computer's memory sequentially, beginning at the starting address:

```rust
/// Locate a program file by filename and load into memory.  Returns total bytes loaded.
pub fn load_game(&mut self, name: &str) -> Result<usize> {
    // Clear the memory to make way
    self.reset();
    let rom = SHELF.rom(name)?; // I have all game data pre-loaded in memory and tagged by name
    // Load in memory starting at location 512 (0x200), which is where the pc pointer starts
    for (idx, &byte) in rom.iter().enumerate() {
        self.memory_set(idx as u16 + self.pc, byte);
    }
    Ok(rom.len())
}
```

It all just gets copied one byte at a time, and the `self.pc` pointer is used to keep track of where the machine is currently looking.  However, each instruction is actually more than a single byte long.  There is a [full list](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table) on Wikipedia, 35 in total.  Here's a few examples:

```txt
1NNN -> Jump to address NNN
8XY4 -> Add the value in register Y to the value in register X
3XNN -> Skip the next instruction if the value in register X equals 0xNN
```

Each of these digits is hexadecimal so each opcode is 2 bytes long, or 16 bits.  In Rust, I use the `u16` datatype to work with these numbers.  Further, each one carries different amounts of information.  There is a hardcoded prefix and/or suffix to determine which exact instruction, and then within that various nibbles are used to denote different things.  Look at the differences between these codes:

```txt
0x1234 -> Jump to address 0x234
     1       2         3        4
0 0 0 1 | 0 0 1 0 | 0 0 1 1 | 0 1 0 0 

Extract value 0x234 - pad to 0x0234
0 0 0 0 0 0 1 0 0 0 1 1 0 1 0 0
___

0x8234 -> Add the value in register 3 to the current value in Register 2
    8       2         3        4
1 0 0 0 | 0 0 1 0 | 0 0 1 1 | 0 1 0 0 

Extract values 0x2 and 0x3
0 0 1 0
0 0 1 1

___

0x3234 -> Skip the next instruction if the value in register 2 equals 0x34
    3       2         3        4
0 0 1 1 | 0 0 1 0 | 0 0 1 1 | 0 1 0 0

Extract values 0x2 and 0x34
0 0 1 0
0 0 1 1 0 1 0 0
```

The only difference in bit-level representation between 0x1234 and 0x8234 is which bit is flipped in the first nibble.  However, the information stored is not used in the same way.  To successfully address all 4096 memory locations a single byte is insufficient, but 12 bits is enough, so we pull the final three nibbles and understand them as a single number.  To specify registers, though, there's only 16 of those - we don't need all the extra space.  A single nibble is enough to uniquely address any of the registers.  Thus, the second and third nibbles are used to specify the targets and the final nibble is simply used as another hard-coded opcode designator.  The `3XNN` code uses a third pattern, extracting two values but one is a nibble and the other is a full byte.  It skips an instruction of the value at register X is equal to the value in NN.

The point is that while specific patterns of bits may be the same or similar, how they're understood within the program can vary.  There's a lot of different ways to efficiently utilize a pattern of 16 bits depending on how many total possibilities there are for each unit of information.  This means that you can't approach an opcode as a single number or even two separate bytes.  It's really four nibbles, and what you do with them will depend on the specific opcode.  It makes most sense to work with these as a pattern of individual bits.

## Bitwise Manipulation

Bitwise operators look at the individual bits in each operand to produce a result, instead of dealing at the higher abstraction level that "numbers" introduce.  We'll look at `&`, `|`, `<<`, and `>>`, with some passing mention of `^` as well.

### Extract Specific Nibbles

The first example was `1NNN`.  The relevant information the machine needs to execute this opcode consists of the last three nibbles of the opcode.  After we have determined that `0x1234` is an opcode of this type, all we need to do is grab that `0x234` value.  To do this you can use the bitwise `&`, or AND:

```
AND
A  B  Result

0  0  0
1  0  0
0  1  0
1  1  1
```

The `AND` operation checks if both values are set to `1`, or `true`.  If they are, `A & B = 1`.  If they aren't both set, the result is `0`, or `false`.  We can use this to find the set bits of only a portion of a 16-bit value.  If we take `0x1234 & 0x0FFF`, the result will contain the exact bits that are set in the last three nibbles and completely ignore the first nibble:

```txt
0x1234 -> 0 0 0 1 0 0 1 0 0 0 1 1 0 1 0 0
0x0FFF -> 0 0 0 0 1 1 1 1 1 1 1 1 1 1 1 1
&        ________________________________
0x0234 -> 0 0 0 0 0 0 1 0 0 0 1 1 0 1 0 0
```

By applying the AND rules to each pair of bits in sequence, the final total has only zeros for the not-relevant first nibble and successfully retains the information from the rest of the first byte.  We now have a bit pattern that fits neatly into a `u16` containing the extracted information.

We can use the same logic to grab, for instance, the last byte, like we do to pull `0x34` from `0x3234`:

```txt
0x3234 -> 0 0 1 1 0 0 1 0 0 0 1 1 0 1 0 0
0x00FF -> 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1
&        ________________________________
0x0034 -> 0 0 0 0 0 0 0 0 0 0 1 1 0 1 0 0
```

In Rust:

```rust
pub fn last_three_digits(&self) -> u16 {
    self.0 & 0x0FFF
}
```

It's important to note once again that this neat mapping of place value to nibble *only* works with hexadecimal - `0x34` in base 16 is `52` in base 10!  It's easy to get confused.

### Extract Any Nibble

As an extension of the previous concept, many of these opcodes require extracting just the second and/or third nibble, which in memory are actually stored in consecutive bytes.  To get the second you can use `0x0F00`, or `0x00F0` for the third.

You *could* write a separate function for each position, but it's not hard to use bit-shifting to generalize it:

```rust
pub fn nibble_from_left(&self, from_most: u8) -> u8 {
    // Make sure the parameter is legal
    if from_most > 3 {
        panic!(
            "cannot get the {}th nibble from 2-byte number {:#0x}",
            from_most, self.0
        );
    };
    let bits = 4 * (3 - from_most);
    ((self.0 >> bits) & 0xF) as u8
}
```

Let's use this function to grab the `0x2` from `0x8234`.  This function expects the index from the most-significant digit, or the left, starting at 0, so to get at the second nibble the index passed as a parameter will be `1`.

First, we can observe that anything less significant - or to the right - of the nibble in question is totally irrelevant.  We can discard it completely.  We are only interested in a *single* nibble, which means that running our bitwise `&` operator against just `0xF` will be sufficient to pull what we need and discard anything before it.  By using the `>>`, or right-shift operator, we can move the nibble in question right to the end of the number:

```
0x8234 -> 1 0 0 0 0 0 1 0 0 0 1 1 0 1 0 0
>> 8
0x0082 -> 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0
```

Shifting right by one discard the rightmost bit, pads a new 0 bit to the left, and moves everything over.  If we do this eight times we've discarded the last two nibbles and moved the one we're interested in right to the end.  Now we can run the `&`:

```
0x0082 -> 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0
0xF    -> 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1
&      ___________________________________
0x0002    0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
```

Brilliant!  Using `>>` and `&`, we were able to produce a bit pattern that corresponds to the exact digit we need.  The missing piece was how much to shift right - `4 * (3 - from_most);` means that we will always shift bits by a multiple of 4 to always deal with a whole nibble at a time.  We just need to know how many nibbles to shift.  To get the 3rd least significant digit, like this case, we take `4 * (3 - 1) = 4 * 2 = 8`.  This way we can reuse the same logic to pull any of the four nibbles out.  The only thing that changes is how much data at the end we discard - if we want the last nibble after all, this would be `4 * (3 - 3) = 0`, and applying `>> 0` to a number will leave it as-is.

This is used very commonly to parse the opcodes used in the Chip8 machine.  Many of them specifically carry information in the middle two nibbles so I just made a helper function to pull those:

```rust
pub fn middle_nibbles(&self) -> (u8, u8) {
    (self.nibble_from_left(1), self.nibble_from_left(2))
}
```

This will very neatly return `(0x2, 0x3)` when called on `0x8234`.

### Combine Bytes

When we loaded the program - or series of opcodes - into memory, we copied them one single byte at a time.  This means that each opcode is occupying two adjacent memory locations.  We need to wrap them up together and understand that sequence of 16 consecutive bits as a single number.  In other words, we need to find the `u16` you get when you place two `u8` values next to each other.  If you have the bytes `0xAB` and `0xCD`, how do you produce the 16-bit value `0xABCD`?

On paper, we can visually see the solution:

```txt
0xAB   -> 1 0 1 0 1 0 1 1
0xCD   ->                 1 1 0 0 1 1 0 1
0xABCD -> 1 0 1 0 1 0 1 1 1 1 0 0 1 1 0 1
```

You just need to grab a space big enough and put the more significant byte to the left of the less significant byte.  Of course, we have to get there with bitwise operators, though, lacking an in-software pencil and paper.

To isolate a nibble from a longer bit sequence, we used the right-shift `>>` above to shift the nibble we need to the end.  The reverse left-shift operation `<<` also exists, which will discard more significant digits to the left and pad with zeros on the right.  If we shift left by 8 bits we will essentially insert a blank byte:

```txt
0xAB   -> 1 0 1 0 1 0 1 1
<< 8
0xAB00 -> 1 0 1 0 1 0 1 1 0 0 0 0 0 0 0 0
```

Great!  To pop our actual intended second byte in, we can use bitwise `OR`, or `|`.  This has a similar function to `AND` but with the following truth table:

```txt
OR
A  B  Result

0  0  0
1  0  1
0  1  1
1  1  1
```

The difference is that this operator returns `true`/`1` if *either* of the operands is set to `1`. It's only false if both bits are false.  We can use this to combine our newly-expanded first byte with the intended second byte:

```txt
0xAB00 -> 1 0 1 0 1 0 1 1 0 0 0 0 0 0 0 0
0xCD   -> 0 0 0 0 0 0 0 0 1 1 0 0 1 1 0 1
|       _________________________________
0xABCD -> 1 0 1 0 1 0 1 1 1 1 0 0 1 1 0 1
```

This operation just pulls down whatever is relevant from either byte, effectively combining them.  So, first, shift the more significant byte, then OR it against the less significant byte to pull out your single 16-bit value:

```rust
pub fn combine_bytes(byte_one: u8, byte_two: u8) -> u16 {
    (byte_one as u16) << 8 | byte_two as u16
}
```

When the emulator emulates a cycle it reads two bytes starting at the current program counter and combines them using this method to understand the current opcode:

```rust
fn fetch_opcode(&self) -> Result<Opcode> {
    let first_byte = self.current_byte();
    let second_byte = self.memory_get(self.pc + 1);
    Ok(Opcode::new(first_byte, second_byte)?)
}
```

In my implementation the `Opcode::new()` logic contains the call to this internal `combine_bytes` method.

### Misc

These are the bitwise operations I've used in handling Chip8 opcodes, but understanding these operators can lead to some very efficient implementations of some other commonly needed operations.

#### Even or Odd

One handy one to keep under your belt is to check whether a given number is even or odd.  A common idiom many of us reach for is the modulo operator: `byte % 2 == 0`.  If the remainder after dividing by two is 0 then it's an even number.

There's actually two ways (that I know) to do this.  We went over bitwise AND in this article, so I'll cover that - it's similar to the trick we used to extract a nibble.  In a binary representation of a number, all we need to determine whether a number is even is the very last bit.  If it's even, this will be a zero, and if it's odd, this will be a one.  If that observation isn't intuitive, go [back up](https://dev.to/deciduously/using-bitwise-operators-why-waste-space-use-many-bits-when-few-bits-do-trick-2ap6#hexadecimal-representation) and look at the mappings from hex digits to nibbles and see if you can see why this is true.

This means that it's sufficient to AND your value against `1`.  You will get back a 1 if the value to check ends in a 1 bit, or a 0 if it's zero.

```
5 -> 0 1 0 1
1 -> 0 0 0 1
&  _________
1 -> 0 0 0 1

___

6 -> 0 1 1 0
1 -> 0 0 0 1
&  _________
0 -> 0 0 0 0
```

Then, just check what you got.  If the bit was set, it's an odd number, otherwise, it's even.  In Rust:

```rust
fn is_even(val: u32) -> bool {
    val & 1 == 0
}
```

This will work on values of any size, because it only ever looks at the least significant bit.

While in most applications the performance benefit is likely negligible, this solution will be more efficient than using `%`.  In some situations it may be enough to prefer this.

I'll leave the other solution I know as an exercise - can you do it using bitwise XOR (exclusive-or)?  This operator looks like `^` in many languages.  Here's the truth table:

```txt
XOR
A  B  Result

0  0  0
1  0  1
0  1  1
1  1  0
```

The `^`/`XOR` operation is similar to `|`/`OR`, but only returns true if one or the other bit is set and *not* both.  Show me what you get in the comments!

#### Double/Halve Numbers

The left and right shift operations we used above are super handy.  When you push the bits in a binary number to the left, you end up doubling the value:

```
6  -> 0 1 1 0
<< 1
12 -> 1 1 0 0
```

Looking at the above it's clear to see that this will work the other way, too.  Shifting right instead will undo the above, effectively halving the value.   You can extend this to any power of two:

```
6  -> 0 0 1 1 0

<< 2
24 -> 1 1 0 0 0

<< 3
48 -> 1 1 0 0 0 0
```

Shifting right by n is the same thing as multiplying by 2^n.  Neat!  As with the even/odd trick, this will probably be more efficient than multiplying - however, you do lose some readability.  As with any optimization, it always depends on context.

There are numerous other bitwise "hacks" like this.  Share some of your favorites below!

## Acknowledgements

If you're going to tackle your own Chip8 emulator, while the Wikipedia page has a lot of what you need, I also recommend [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) by [Laurence Muller](http://www.multigesture.net/about/).  So far, this blog post and Wikipedia have been the only two resources I've needed.

*Cover image: Photo by [Federica Galli](https://unsplash.com/@fedechanw?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText) on [Unsplash](https://unsplash.com/)*
