# skik - an s-box solving tool written in Rust
Picture this: you're doing a CTF and you come across a bit of ciphertext that, when decrypted, gives you the flag.
You obviously want to be a *l33t h4x0r* and stay in the terminal for all your daily tasks, but there's probably
already someone who made a JavaScript tool for that.  
Now picture a tool that can read that ciphertext from a file or stdin, has an intuitive TUI and, above all, automatic frequency analysis?  
**That's what skik is.**  

Apologies for the sales pitch just then. To give you the non-marketing version: I had an idea for a tool I wanted to make and a language I didn't know (and, to be fair, still don't properly know) and a slightly above average amount of motivation to make something. And thus, skik was born.

## Usage
As mentioned earlier, you can pipe in ciphertext or open a file (which, behind the scenes, just calls `cat file1.txt | skik`) after which you're greeted with this lovely screen:
```
 skik v. 0.1.0a, running in a 83 by 20 terminal.

 -- ciphertext --
 this is some sbmple ciphertext. the quick arown fox jumps over the lbzy dog, b
 nd he does so in such b mbnner thbt it doesn't wbke up sbid lbzy dog. quite im
 pressive!

 -- mapping table --
 [ a > b ] [ b > a ] [ c > _ ] [ d > _ ] [ e > _ ] [ f > _ ] [ g > _ ]
 [ h > _ ] [ i > _ ] [ j > _ ] [ k > _ ] [ l > _ ] [ m > _ ] [ n > _ ]
 [ o > _ ] [ p > _ ] [ q > _ ] [ r > _ ] [ s > _ ] [ t > _ ] [ u > _ ]
 [ v > _ ] [ w > _ ] [ x > _ ] [ y > _ ] [ z > _ ]
```
TODO: add screenshot  
The top section is the text you're working with. This changes on the fly depending on which mode you're in.

## Miscellaneous
The name skik was, much like the name of git, chosen at random. I desperately tried to retcon the name:
```substitution box -> s-box / one can kickbox -> skik```