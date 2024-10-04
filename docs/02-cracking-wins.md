# Cracking wins amount

`game.sys` has this silly values for wins.
Normally fresh game starts with `0` wins, which is logical,
but the values are encoded in the save file.

Using Cheat Engine helped me figure out the game uses
a normal `u32` at runtime to store wins per character.
Neato.

Extra fun fact: wins are in static memory,
so the address of the wins amount persists before runs.

Then a simple diff on a file after obtaining one win
showed the location where wins are stored for this character.

Cracked the algorithm within 3-4 days (a few hours of work a day).

## The numbers

Collected a bunch of numbers for wins.

Encoded values look more like something meaningful in the hex.
Didn't even try looking at decimal values - hex editor showed hex, duh.

The format below is: the left pair is real value of wins and its binary version,
and the right pair is the encoded value after its binary version.

Those were collected manually trying to understand the idea behind it.

```text
000 = 00000000  10001110 = 0x8e
001 = 00000001  10011110 = 0x9e
002 = 00000010  10101110 = 0xae
003 = 00000011  10111110 = 0xbe
004 = 00000100  11001110 = 0xce
005 = 00000101  11011110 = 0xde
006 = 00000110  11101110 = 0xee
007 = 00000111  11111110 = 0xfe
008 = 00001000  00001110 = 0x0e
009 = 00001001  00011110 = 0x1e
010 = 00001010  00101110 = 0x2e
011 = 00001011  00111110 = 0x3e
012 = 00001100  01001110 = 0x4e
013 = 00001101  01011110 = 0x5e
014 = 00001110  01101110 = 0x6e
015 = 00001111  01111110 = 0x7e
016 = 00010000  10001111 = 0x8f
017 = 00010001  10011111 = 0x9f
018 = 00010010  10101111 = 0xaf
019 = 00010011  10111111 = 0xbf
020 = 00010100  11001111 = 0xcf
021 = 00010101  11011111 = 0xdf
022 = 00010110  11101111 = 0xef
023 = 00010111  11111111 = 0xff
024 = 00011000  00001111 = 0x0f
025 = 00011001  00011111 = 0x1f
026 = 00011010  00101111 = 0x2f
027 = 00011011  00111111 = 0x3f
028 = 00011100  01001111 = 0x4f
029 = 00011101  01011111 = 0x5f
030 = 00011110  01101111 = 0x6f
031 = 00011111  01111111 = 0x7f
032 = 00100000  10001100 = 0x8c
```

Gathered some bigger values with Cheat Engine.

```text
000 = 00000000  10001110 = 0x8e
016 = 00010000  10001111 = 0x8f
032 = 00100000  10001100 = 0x8c
048 = 00110000  10001101 = 0x8d
064 = 01000000  10001010 = 0x8a
080 = 01010000  10001011 = 0x8b
096 = 01100000  10001000 = 0x88
112 = 01110000  10001001 = 0x89
128 = 10000000  10000110 = 0x86
144 = 10010000  10000111 = 0x87
160 = 10100000  10000100 = 0x84
176 = 10110000  10000101 = 0x85
192 = 11000000  10000010 = 0x82
208 = 11010000  10000011 = 0x83
224 = 11100000  10000000 = 0x80
240 = 11110000  10000001 = 0x81
```

## Observations

Imagine a byte with bits numbered as follows:

```text
12345678
```

First thing I noticed was that the bits `678` in the original
are equivalent to bits `234` in the encoded value.

Then looking at big values I saw that original bit `5` does not change,
and encoded bit `1` doesn't change either.
Double checked with the smaller values - turned out, they're the inverse of each other.

At this point I realized that four lowest bits are moves to highest.
Therefore, highest bits are likely moved to lowest.

So after that I started to comparing four highest bits of the original
to 4 lowers bits of the encoded number.
First thing I noticed is that the rightmost (bit `4` in the original
and bit `8` in the encoded number) are identical.
And the last thing that was then easy to notice is that bits `123` in the source
are the inverse of bits `567` in the target.

## The algorithm

Let's number the bits in the byte for simplicity:

```text
12345678
```

Let's split the bits into sections

```text
123 4 5 678
```

Then reorder bits as follows.
This is equivalent to shifting lowest half 4 bits left, and highest half 4 right right,
essentially swapping them

```text
5 678 123 4
```

Then let's apply bitwise negation to some of the sections (marked with `!` here).

```text
!5 678 !123 4
```

This is the wat to encode the number.
