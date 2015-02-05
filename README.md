## Shannon Entropy calculation, from file or string inputs.

This is a straightforward implementation of a basic Shannon Entropy calculator in Rust.   
[Wikipedia entry on Shannon Entropy](http://en.wikipedia.org/wiki/Entropy_%28information_theory%29)    

 Reads bytes from files and characters from strings, calculates the result appropriately.    
 Fairly quick; should churn though 100MB+ files in less than a second.   


## howto
Run from command line, passing in a string or, with the -f flag, a filename.   
-h or --help for help.   
Returns the basic Shannon Entropy in bits.   

#### examples

Shannon Entropy of the string "1223334444" (also contained in the demo.txt file)   

shannon 1223334444     


or via file:   
shannon -f demo.txt ->    

  character count: 10   
  value: 1.84643934467    
 

## performance  
Just for kicks, here is an informal speed comparison of this code to a mostly similar [OCaml version](https://github.com/jme/shannon) I had previously written:     


#### prep 

* Rust version: rust-1.0.0-nightly (c5961ad06)  
* OCaml version: 4.01.0  

* rust executables were built with 'cargo --release'  
* ocaml builds were built using the -compact option  

* target data file for analysis: [rust-nightly-x86_64-unknown-linux-gnu.tar.gz (~116MB)](   http://static.rust-lang.org/dist/2015-01-04/rust-nightly-x86_64-unknown-linux-gnu.tar.gz)  

* all runs were made on the same (fairly low-end) laptop, under Ubuntu 14.04.1 LTS  

* all run elapsed times are *real* durations as measured using the unix *time* command.  

* profile data collected with [Oprofile](http://oprofile.sourceforge.net/news/): "operf *filename*, then: "opreport -l" and "opreport --callgraph"  


#### results

given times are in seconds and are each the average over three runs  

   ----------------------------
style A: using HashMap (Rust) / Hashtbl (OCaml) for the bins   

shannon-rust    : 21.940s   
shannon-ocaml   : 51.469s    

   ----------------------------
style B: using mutable arrays for the bins  

shannon-rust    : 0.612s   
shannon-ocaml   : 2.435s    



## discussion
The HashMap / Hashtbl constructs are inappropriate for speed-oriented binning in the manner done here, but are included for purposes of comparison.   

> However, one option to speed up the OCaml version would be to build a char-specific Hashtbl via the Hashtbl functorial interface, with char-appropriate compare, equality (and maybe hash) functions  

Rust, and its std libs are evolving very rapidly right now so that HashMap performance may be in flux as well.  

The array binning performance differences are slightly surprising. There are further possible tweaks to the OCaml code (unboxing, write-barrier avoidance, etc) that might tighten up the spread, but I have skipped that deeper dive for now.  

Oprofile runs for all 4 variants ([rust-array](perf/perf_rust_array.txt), [rust-hashmap](perf/perf_rust_hm.txt), [ocaml-array](perf/perf_ocaml_array.txt) and [ocaml-hashtbl](perf/perf_ocaml_hm.txt) ) are unsurprising: Hashtbl/HashMap versions spend most of their time in Hash structure operations, while the array versions are limited more by memory and file-read ops.  

 
There is a stale joke to the effect that speed-run optimizations of code written in FP style lead right back to imperative-land. But I kind-of did do this to the OCaml code, whittling down the file reader into something mutable. Hopefully not so much Crude Hackery as 'pragmatic FP'.  
The Rust variant was patterned after the OCaml code.   

Even so, for **_this_** quasi-toy program the Rust version(s) run at 2-4x the speed of **_similar_** OCaml version(s).  


  
   
 Rust seems to have an interesting idiomatic style, although these are still early days and my experience with the language is minimal. The ML heritage certainly is there, as is the C/C++ feel. It's not really a functional programming language but still feels comfortable to someone who writes Clojure (and some OCaml) code most of the time. And it's fast.   
   
Unfortunately this toy-like code doesn't really dig into traits or the innovative ownership concepts & borrow-checker.  Obviously I need to write some more Rust code :-)   



## License

Copyright © 2015 jm ervin

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



