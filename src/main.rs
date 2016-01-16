/// Shannon Entropy calculator (rust 1.5+)

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::collections::HashMap;



// read bytes from file and bin the values using mutable array bins 
// and a raw array read-buffer.
// (somewhat old school imperative with this IO style..).
//
fn bins_of_file_bytes(fname: String) -> Option<[u64; 256]> {

  // a structurally hacky flag that just indicates that xbins contains *something* 
  let mut gotsome = false;

  let mut xbins  = [0; 256];

  match File::open(fname) {
    Ok(f) => {
              let mut file = BufReader::new(f);

              // just an arbitrary buffer size
              const BUFFSIZE: usize = 400000;

              let mut buf  = [0; BUFFSIZE];
              gotsome = false;

               loop {

                  // also, quick note: by using a slice here we don't have to bother with
                  // clearing the buffer between loop calls as the slice
                  // ONLY includes newly-read bytes.
                  match file.read(&mut buf) {
                    Ok(nread) => { for k in (&buf[0..nread]).iter() {

                                     let foof = *k as usize;

                                     xbins[foof] = xbins[foof] + 1; 
                                  }
                                  gotsome = true;
                                  if nread == 0 {break}; }, 

                    Err(e) =>  { println!("\nfile read error: {:?}", e); 
                                break },
                  } 
               }
             },
  
    Err(e) => { println!("\nfile open error: {:?}", e); },
  }  

 if gotsome {Some(xbins)} else {None}
}



// the actual entropy calculation,
// this fn is later mapped across a collection of bins. 
//
fn fproc(x:f64, file_length:f64) -> f64  {
   
   let divx = x / file_length;

   match x {
     0f64 => 0f64,
     _    => { divx * divx.log2() }
   }
}



// primary core fn for calculation via file input.
// what it does: reads input and bins chars. Folds-in the entropy calculation
// and returns result + file length in a tuple.
//
fn shannon(s : String) -> Option<(f64, f64)> {

   match bins_of_file_bytes(s) {
   
     Some(bins) => { let file_length = 
                              bins.iter()
                                  .fold( (0.0 as f64),
                                         |a, b| a + (*b as f64)); 

                     let entropy = 
                             -1f64 * bins.iter()
                                         .fold( (0.0 as f64), 
                                                |a, b| a + fproc((*b as f64), 
                                                                 file_length ));

                     Some( (entropy, file_length) ) 
                   }
     None => None 
   }
}


// generate a bin set for a String input
// slightly different method than for the file input case
// returns a HashMap full of bin data for the string input
//
fn bins_of_string (instr:  String) -> HashMap<u8, i64> {

  let mut xbins : HashMap<u8, i64> = HashMap::new();

  let cbox = instr.as_bytes();

  for c in cbox {
    if xbins.contains_key(c) {

        match xbins.get_mut(c) {
          Some(x) => {*x = *x + 1},
          None => (), } }

    else { xbins.insert(*c, 1i64); }
  } 

 xbins
}




// core fn for calculation via string input
// what it does: just folds the entropy calculation into the string.
// Unlike the file-input core fn the result here is returned in a
// regular tuple, rather than an Option. 
//
fn shannon_str(instr : String) -> (f64, f64) {

   println!("param: {}", instr);
   println!("param length: {}", instr.len());

   let string_length = instr.len() as f64;
   let bins = bins_of_string(instr);

   let entropy = -1f64 * bins.iter()
                             .fold( (0.0 as f64), 
                                    |a, (_, val)| a + fproc((*val as f64), 
                                                            string_length ));
  (entropy, string_length)
}




// IO: display usage message
//
fn display_usage() -> () {
  println!("\nUsage: shannon somestring \n   or: shannon -f filename");
}



// IO: combine error display with usage message
// accretes error msg and output of a HOF
//
fn helperr<F>(msg: &str, fx:F  )
          where F: Fn() -> ()
{
  println!("{}", msg);
  fx();
}



// IO: DWISOTT
//
fn display_results(results : (f64, f64)) -> () {

   let (entropy, count) = results;

   println!("\ncharacter count:  {}", count);
   println!("shannon entropy: {:15.12}", entropy);
}



// entry point. map arguments to appropriate handler functions.
// probably there is now a nice param options lib out there somewhere...
//
fn main() {

  // some gratuitous indirection, just for testing & comparing the 
  // various file and binning methods

  // unnecessary here, a vestige of the older rust alpha + beta versions
  // that compared various IO styles and bin containers 
  let shannon_handler = shannon;


   match env::args().len() {

     2 => { display_results( shannon_str (env::args().nth(1).unwrap()));},

     3 => { match shannon_handler(env::args().nth(2).unwrap()) {

                       Some((x, i)) =>  { display_results((x, i)); },

                       None => helperr("", display_usage ),
                    }
          },

     _ => display_usage(), 
   } 

   println!("--------------------");
}






