# Shannon Entropy calculation, from file or string inputs.

This is a straightforward implementation of a basic Shannon Entropy calculator in Rust.   
[Wikipedia entry on Shannon Entropy](http://en.wikipedia.org/wiki/Entropy_%28information_theory%29)    

 Reads bytes from files and characters from strings, calculates the result appropriately.    
    
 

## howto
Run from command line, passing in a string or, with the -f flag, a filename.   
-h or --help for help.   
Returns the basic Shannon Entropy in bits.   

Fairly quick; release builds should churn though 100MB+ files in less than a second.   


## examples

Shannon Entropy of the string "1223334444" (also contained in the demo.txt file)   

shannon 1223334444     

or via file:   
shannon -f demo.txt ->    

  character count: 10   
  value: 1.84643934467    
 


## comments:
 Rust seems to have an interesting idiomatic style, although these are still early days and my experience with the language is minimal. The ML heritage certainly is there, as is the C/C++ feel. It's not really a functional programming language but still feels comfortable to someone who writes Clojure (and some OCaml) code most of the time.   
   
Unfortunately this toy-like code doesn't really excercise the innovative ownership concepts & borrow-checker.    



## Prerequisites `

 The Rust language is still undergoing rapid development; this code is circa Rust-alpha-1.0.0     



## License

Copyright © 2014-2015 jm ervin

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



