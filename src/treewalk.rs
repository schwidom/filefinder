
use std::path::PathBuf;
use std::fs::read_dir;
use std::fs::ReadDir;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::process::exit;

pub struct TreeWalk
{
 path : VecDeque<PathBuf>,
 next_dir : Option<PathBuf>,
 excluded_files : HashSet<PathBuf>, // darf nicht mit ./ beginnen
}

impl TreeWalk {
 pub fn new( path : PathBuf) -> Self { TreeWalk{ path : VecDeque::from([path]), next_dir : None, excluded_files : HashSet::new() }}
 pub fn cut( &mut self) { self.next_dir = None;}
 pub fn inject( &mut self, path : PathBuf) {
  self.path.push_back( path);
 }
 pub fn insert_excluded_filename( &mut self, fname : PathBuf) {
  if PathBuf::from( "") == fname { return } // sonderfall vom splitten
  self.excluded_files.insert( fname);
 }
}

impl Iterator for TreeWalk {
 type Item = PathBuf;

 fn next( &mut self) -> Option<<Self as Iterator>::Item> {

  if let Some( dir ) = &self.next_dir {
   if let Ok( dir2) = read_dir( dir) {

    for file in dir2 {

     let path = file.unwrap().path();

     if ! self.excluded_files.contains(&path) {
      self.path.push_back( path);
     }
    }
   }
  }

  let ret = self.path.pop_front();

  if let Some( path) = &ret {
   // assert!( !( path.is_dir() &&  path.is_symlink())); // happens
   // in order to not stay in loops, links are not followed // jmu1xuoi8c
   // TODO : implement explicit link following mechanism
   self.next_dir = if path.is_dir() && ! path.is_symlink() { Some( path.clone()) } else { None };
  }

  ret
 }
}

// TODO : create os independend test directory
pub fn tests() {

 {
  let tw = TreeWalk::new( PathBuf::from( "/home/ox/tmp/2022_11_25_find_exec_test"));
  for x in tw { 
   println!("{}", x.display());
  }
 }

 println!();

 {
  let mut tw = TreeWalk::new( PathBuf::from( "/home/ox/tmp/2022_11_25_find_exec_test"));
  loop { 
   match tw.next() {

    Some( x) => {
      println!("{}", x.display());
      if x == PathBuf::from( "/home/ox/tmp/2022_11_25_find_exec_test/b") { tw.cut();}
     } ,
    None => break
   }
  }
 }

 /*
 {
  // ohne den softlink schutz jmu1xuoi8c verbraucht er ein haufen Speicher und wird nicht fertig
  // verbrauchte für alle Files immer nur 0.7% (224 MB) als debug target
  // load average lag zwischen 1.18 und 1.50
  let tw = TreeWalk::new( PathBuf::from( "/"));
  for x in tw { 
   println!("{}", x.display());
  }
 }
 */


 /*
 {

  /*
 
   Es gibt definitiv noch Optimierungsbedarf

   (ins)$ time ./target/debug/filefinder -r | wc -l # ohne die obigen ausgaben
   1440728
   
   real	0m6,919s
   user	0m3,094s
   sys	0m7,433s
   
   
   (ins)$ time find /usr/ | wc -l
   find: ‘/usr/share/horde/.git’: Keine Berechtigung
   1440728
   
   real	0m1,966s
   user	0m0,884s
   sys	0m1,177s



   (ins)$ time ./target/debug/filefinder -r >/dev/null 
   
   real	0m5,014s
   user	0m2,114s
   sys	0m2,883s

   (ins)$ time ./target/release/filefinder -r >/dev/null 
   
   real	0m4,545s
   user	0m1,581s
   sys	0m2,947s

  */

  let tw = TreeWalk::new( PathBuf::from( "/usr"));
  for x in tw { 
   println!("{}", x.display());
  }
 }
 
 */

}

