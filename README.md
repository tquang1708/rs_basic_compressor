# rs_basic_compressor
Basic compressor in Rust, made as the final for the Info Theory tutorial at Bennington College Spring 2021. Using LZW.

Ran some test files (in the testfile folder). Works great with texts: the Tom Sawyer all ASCII text file saw a reduction to only about 40% of the original, though it doesn't work at all with images. Both images I tried it on actually saw an increase in size, and for some reason one of the image after compressing/decompressing has a red tint to the image.

I'm not fixing it though since I wrote it in 4 days and today's the due date. Was planning to pipe LZW into Huffman too but oh well. How I set it up though that wouldn't be too hard - just pipe the output. I just don't feel like doing it.

Bit manip was fun though, when it works.
