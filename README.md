# Summarizing what I learned

The chip-8 is more of an interpreter than an emulator. Still shaky on this subject.

First start by opening the file. Using read to end because it reads 8 bits at a time and stores them into an array.

struct has:
memory: array of 8 bit integers. 4096 bytes because every byte is 8 bits.

Addresses are 16 bits. When we use addresses, we store them as such 

Now that memory should be in the same module as the chip8.  Will fix this later.

The memory from 0 to 512 holds the sprites, which is an array of 16 elements. Each element is an array of 5 bytes, representing hexadecimal digits from 0 - F.

Use array splicing if you want to use i in array indexing.

Chip-8 instructions are stored big-endian

using u8 for the 4 bit values

CHECK THE OPCODES CAREFULLY They say things like "carry flag is not set", which causes errors. I was stuck on this one bug for hours 



https://stackoverflow.com/questions/40760168/how-to-handle-errors-from-the-readread-to-end-method



http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
https://doc.rust-lang.org/book/second-edition/

