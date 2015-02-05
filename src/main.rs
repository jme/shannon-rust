// Shannon Entropy calculation, from file or string inputs.
// Includes a few source variations on file-read and binning operations,
// but uses the fastest variation.
  
use std::collections;
use std::num::Float;
use std::os;
use std::old_io;


// note: the unicode-friendly graphemes do not include newlines.  
// using a regular HashMap for bins 
//
fn read_str(instr: &str) -> collections::HashMap<&str, int> {

  let mut xbins : collections::HashMap<&str, int> = collections::HashMap::new();
    
    for c in instr.graphemes(true){

      if xbins.contains_key(c) {

          match xbins.get_mut(c) {

            Some(x) => {*x = *x + 1},
            None => (), } }

      else { xbins.insert(c, 1i); }
     }
 xbins
}



// read bytes from file and bin the values 
// one of several variations, this one uses HashMap bins + vector read-buffer. 
// 
fn readem_bytes_hm(fname: &str) -> Option< collections::HashMap<u8, int> > {

  let mut xbins : collections::HashMap<u8,int> = collections::HashMap::new();
  let path = Path::new(fname);
  let mut file = old_io::BufferedReader::new(old_io::File::open(&path));
  let mut buf:Vec<u8> = Vec::new();
  let buffsize = 400000;

  // this is a cache of items to be added to the main hash 
  let mut fngs:Vec<u8> = Vec::new();

  // idempotent flag: just want to indicate that the xbins has
  // *something* in it...
  let mut gotsome = false;

   loop {
      match file.push(buffsize, &mut buf) {

        Ok(nread) => { for z in  (buf.slice(0, nread)).iter() {

                            match xbins.get_mut(z) {
                              Some(x) => { *x = *x + 1; },
                              None => {fngs.push(*z); },
                            } 

                            // add in any new additions to the bins...
                            for k in fngs.iter() {
                              xbins.insert(*k, 1i); 
                            }
                            fngs.clear();
                        }
                        gotsome = true; }, 

        // display all error messages except the 'normal' EOF flag.
        Err(e) =>  {if e.kind != std::old_io::IoErrorKind::EndOfFile 
                       {println!("\nfile read error: {}", e.desc);}

                    break},
      } 

   // empty out the buf for the next loop
   buf.clear(); 
   }

 if gotsome {Some(xbins)} else {None}
}



// read bytes from file and bin the values 
// one of several variations, this one uses mutable array bins + vector read-buffer. 
//
fn readem_bytes_vector(fname: &str) -> Option<[uint; 256]> {

  let mut xbins  = [0u; 256];
  let path = Path::new(fname);
  let mut file = old_io::BufferedReader::new(old_io::File::open(&path));
  let mut buf:Vec<u8> = Vec::new();
  let buffsize = 400000;

  // idempotent flag: just want to indicate that the xbins has
  // *something* in it...
  let mut gotsome = false;

   loop {
      match file.push(buffsize, &mut buf) {

        Ok(nread) => { for k in (buf.slice(0, nread)).iter() {

                         let foof = *k as uint;
                          xbins[foof] = xbins[foof] + 1; 
                      }
                      gotsome = true; }, 

        // display all error messages except the 'normal' EOF flag.
        Err(e) =>  {if e.kind != std::old_io::IoErrorKind::EndOfFile 
                       {println!("\nfile read error: {}", e.desc);} 
                    break},
      } 

   // empty out the buf for the next loop
   buf.clear(); 
   }

 if gotsome {Some(xbins)} else {None}
}



// read bytes from file and bin the values 
// last of the variations, this one uses mutable array bins 
// and a raw array read-buffer.
// (getting somewhat old school with this IO style..).
//
fn readem_bytes_buffer(fname: &str) -> Option<[uint; 256]> {

  let mut xbins  = [0u; 256];
  let path = Path::new(fname);
  let mut file = old_io::BufferedReader::new(old_io::File::open(&path));

  const BUFFSIZE: uint = 400000;
  let mut buf  = [0; BUFFSIZE];


  // idempotent flag: just want to indicate that the xbins has
  // *something* in it...
  let mut gotsome = false;

   loop {

      // also, quick note: by using a slice here we don't have to bother with
      // clearing the buffer between loop calls as the slice
      // ONLY includes newly-read bytes.
      match file.read(&mut buf) {
        Ok(nread) => { for k in (buf.slice(0, nread)).iter() {

                         let foof = *k as usize;
                          xbins[foof] = xbins[foof] + 1; 
                      }
                      gotsome = true; }, 

        // display all error messages except the 'normal' EOF flag.
        Err(e) =>  {if e.kind != std::old_io::IoErrorKind::EndOfFile 
                       {println!("\nfile read error: {}", e.desc);} 
                    break},

      } 
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
fn shannon(s : &str) -> Option<(f64, f64)> {

  // the speed of both of these two file input functions is about equal.
  // readem_bytes_vector and 
  // readem_bytes_buffer 

   match readem_bytes_buffer(s) {
   
     Some(bins) => { let file_length = 
                              bins.iter().fold( (0.0 as f64), |a, b| a + (*b as f64)); 

                     let entropy = 
                             -1f64 * bins.iter().fold( (0.0 as f64),
                                               |a, b| a + fproc((*b as f64), file_length ));

                     Some( (entropy, file_length) ) 
                   }
     None => None 
   }
}



// benchmarking comparison core fn. Uses the HashMap reader.
// too slow (relatively) for production.
//
fn shannon_hm(s : &str) -> Option<(f64, f64)> {

   match readem_bytes_hm(s) {
   
     Some(bins) => { let file_length = 
                              bins.iter().fold( (0.0 as f64), 
                                                 |a, (_, b)| a + (*b as f64)); 

                     let entropy = 
                             -1f64 * bins.iter().fold( (0.0 as f64),
                                                  |a, (_, b)| a + fproc((*b as f64),
                                                  file_length ));

                     Some( (entropy, file_length) ) 
                   }
     None => None 
   }
}



// core fn for calculation via string input
// what it does: just folds the entropy calculation into the string.
// Unlike the file-input core fn the result here is returned in a
// regular tuple, rather than an Option. 
//
fn shannon_str(instr : &str) -> (f64, f64) {

   let bins = read_str(instr);
   let string_length = instr.len() as f64;

   let entropy = -1f64 * bins.iter()
                        .fold( (0.0 as f64), |a, (_, val)| a + 
                                                              fproc((*val as f64), 
                                                                     string_length ));
  (entropy, string_length)
}



// IO: display usage message
//
fn display_usage() -> () {
  println!("\nUsage: shannon somestring \n   or: shannon -f filename");
}



// IO: combine err display with usage message
// accretes err msg and another IO HOF; hmmm...syntax.
//
fn helperr<F>(fname: &str, fx:F  )
          where F: Fn() -> ()
{
  println!("Unknown file: {}\n--------------------", fname);
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
// probably there is a nice param options lib out there somewhere...
//
fn main() {

  // some gratuitous indirection, just for testing & comparing the 
  // various file and binning methods
  let shannon_handler = shannon;


   match os::args().len() {

     2 => { display_results( shannon_str(os::args()[1].as_slice()) ); },

     3 => { match os::args()[1].as_slice() {

             "-f" => match shannon_handler( os::args()[2].as_slice() ) {

                       Some((x, i)) =>  { display_results((x, i)); },

                       None => helperr(os::args()[2].as_slice(), display_usage ),
                     },

              _ => display_usage(),
           } },

     _ => display_usage(), 
   } 

   println!("--------------------");
}

