
extern crate clap;
use clap::Parser;

extern crate sexp;
use sexp::Sexp;
use sexp::Atom;

use std::path::Path;
use std::path::PathBuf;

use std::process::exit;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

use std::ffi::OsString;
use std::ffi::OsStr;

// use std::fs::File;
use std::fs::read_to_string;

extern crate regex;

fn get_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[path = "treewalk.rs"]
mod treewalk;

#[derive(Debug,Parser,Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
 #[arg(short, long)]
 path : Option<String>,

 // #[arg(short, long)]
 // runtests : bool,

 #[arg(short, long)]
 expression : Vec<String>,

 #[arg(short, long)]
 check_expression : Vec<String>,

 // #[arg(short = 'i', long)]
 // help2 : bool, // optional und ohne Parameter
 
 #[arg(long)]
 exclude_from_file : Vec<String>, // darf nicht mit ./ beginnen
}

fn runtests() {
 treewalk::tests();
}


trait SexpOrVec {}

impl SexpOrVec for Sexp {}
impl SexpOrVec for Vec<Sexp> {}
impl SexpOrVec for &[Sexp] {} // slice

#[derive(Debug,Clone)]
struct State<'a,T> where T : SexpOrVec {
 // help : bool,
 path : Option<&'a PathBuf>,
 stmt : T, // Sexp | Vec<Sexp>
}

#[derive(Default)]
struct TreeWalkMethods{
 cutted : bool,
 injected : VecDeque<PathBuf>,
 already_injected : HashSet<PathBuf>,
}

impl TreeWalkMethods {
 fn new() -> TreeWalkMethods { TreeWalkMethods::default()}
 fn cut( &mut self) { self.cutted = true;}
 fn uncut( &mut self) { self.cutted = false;}
 fn inject( &mut self, path : &PathBuf) { self.injected.push_back( path.clone());}
 fn injectonce( &mut self, path : &PathBuf) {
  if ! self.already_injected.contains(path) {
   self.already_injected.insert( path.clone());
   self.injected.push_back( path.clone());
  }
 }
 fn transmit( &mut self, tree_walk : &mut treewalk::TreeWalk) {
  if self.cutted { tree_walk.cut(); }
  self.cutted = false; // fix (wichtig)
  while let Some( x) = self.injected.pop_front() {
   tree_walk.inject(x);
  }
 }
}

trait ComparatorTrait<T> {
 fn cmp( &mut self, s1 : &Sexp, s2 : T) -> bool;
}

#[derive(Default)]
struct Comparator {
  regex_map : HashMap<String,regex::Regex>,
}

impl ComparatorTrait<&String> for Comparator {
 fn cmp( &mut self, s1 : &Sexp, s2 : &String) -> bool
 {
  // if let Sexp::Atom( Atom::S(path)) = &state.stmt[1]

  match s1 {
   Sexp::Atom( Atom::S( value1)) => value1 == s2,
   _ => panic!(),
  }
 }
}

impl Comparator {
 fn handle_regex( &mut self, regex_stmt : & Vec<Sexp>, subject_str : &String) -> bool {

  if 2 == regex_stmt.len() && Sexp::Atom( Atom::S( "regex1".to_string())) == regex_stmt[0] {
   if let Sexp::Atom( Atom::S( regex_str)) = &regex_stmt[1] {

    if ! self.regex_map.contains_key( regex_str) { 
     self.regex_map.insert( regex_str.clone(), regex::Regex::new(regex_str.as_str()).unwrap());
    }

    let regex = &self.regex_map[regex_str]; // copy

    return regex.is_match( subject_str.as_str())
   }
  }
  panic!();
 }
}

impl ComparatorTrait<&OsString> for Comparator {
 fn cmp( &mut self, s1 : &Sexp, s2 : &OsString) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => &OsString::from( value1) == s2,
   Sexp::List( stmt) => self.handle_regex( &stmt, &s2.to_string_lossy().to_string()),
   _ => panic!(),
  }
 }
}

impl ComparatorTrait<&OsStr> for Comparator {
 fn cmp( &mut self, s1 : &Sexp, s2 : &OsStr) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => &OsString::from( value1) == s2,
   Sexp::List( stmt) => self.handle_regex( &stmt, &s2.to_string_lossy().to_string()),
   _ => panic!(),
  }
 }
}

#[derive(Default)]
struct Interpreter {
 tree_walk_methods : TreeWalkMethods,
 comparator : Comparator,
}

use std::fs::metadata;

trait PathBufTrait {
 fn is_empty( &self ) -> bool;
}

impl PathBufTrait for PathBuf {
 fn is_empty( &self) -> bool { 
  if self.is_file() {
   match metadata( self) {
    Ok( md) => md.len() == 0,
    _ => false,
   }
  } else if self.is_dir() {
   self.read_dir().unwrap().next().is_none()
  } else {
   false
  }
 }
}

impl Interpreter {

 fn new() -> Self { Interpreter::default() }

 fn interpret_term( &mut self, state : State<Sexp>) -> bool {

  match &state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "t" => true,
     "f" => false,
     "cut" => { self.tree_walk_methods.cut(); true},
     "uncut" => { self.tree_walk_methods.uncut(); true},
     "inject" => { if let Some( path) = state.path { self.tree_walk_methods.inject(path);} true},
     "isdir" => state.path.unwrap().is_dir(),
     "isfile" => state.path.unwrap().is_file(),
     "islink" => state.path.unwrap().is_symlink(),
     "exists" => state.path.unwrap().exists(),
     "isempty" => state.path.unwrap().is_empty(),
     _ => panic!( "{}", "not implemented3 : ".to_string() + atom),
   }},

   Sexp::List( list) => {
    self.interpret_slice( State::<&[Sexp]>{ stmt : &list[..], path : state.path })
   },

   _ => panic!("not implemented4")
  }
 }

 fn interpret_slice( &mut self, state : State<&[Sexp]>) -> bool {
  if state.stmt.is_empty() { return true;}

  if let Sexp::Atom(Atom::S( atom)) = &state.stmt[0] {
   match atom.as_str() {
    // TODO : hashtable
    // "help" => true, // TODO
    "ct0" => true, // comment true
    "cf0" => false, // comment false
    "t0" => {
     self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path });
     true
    },
    "f0" => {
     self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path });
     false
    },
    "or0" => state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "and0" => state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "progn0" => state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "not0" => { 
      ! self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path })
    }, 
    "do0" => {
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..], path : state.path})
    },
    "cut0" => { 
      self.tree_walk_methods.cut(); 
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..], path : state.path})
    },
    "uncut0" => {
       self.tree_walk_methods.uncut();
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..], path : state.path})
    },
    "inject1" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path));
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path })
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
/* // planned feature
    "injectformula" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(PathBuf::from( path));
       true
      } else {
       panic!("string expected") 
      }
    }, 
*/
    "injectonce1" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.injectonce( &PathBuf::from( path));
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path })
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
       newpath.push(PathBuf::from(path));
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..], path : Some( &newpath)})
      } else { 
       panic!("error in {}: string expected", atom)
      }
    },
    "inback0" => {
      let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
      newpath.pop();
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : Some( &newpath)})
    },
    "dirname1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().parent().unwrap().as_os_str()) &&
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..].to_vec() , path : state.path })
    }, 
    "path1" => { 
       self.comparator.cmp( &state.stmt[1], state.path.unwrap().as_os_str()) &&
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..].to_vec() , path : state.path })
    }, 
    "basename1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().file_name().unwrap()) &&
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..].to_vec() , path : state.path })
    }, 
    "filestem1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().file_stem().unwrap()) &&
      self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..].to_vec() , path : state.path })
    }, 
    "extension1" => { 
      if let Some( ext) = state.path.unwrap().extension() {
       self.comparator.cmp( &state.stmt[1], ext) &&
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..].to_vec() , path : state.path })
      } else {
       false
      }
    }, 
    "isdir0" => { state.path.unwrap().is_dir() && self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : state.path }) },
    "isfile0" => { state.path.unwrap().is_file() && self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : state.path }) },
    "islink0" => { state.path.unwrap().is_symlink() && self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : state.path }) },
    "exists0" => { state.path.unwrap().exists() && self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : state.path }) },
    "isempty0" => { state.path.unwrap().is_empty() && self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : state.path }) },
    _ => panic!("not implemented: ''{}''", atom),
   }
  } else {
   panic!("not implemented5 ''{}''", &state.stmt[0])
  }
 }

}

fn main() {
 
 let args = Args::parse();
 let args2 = args.clone();

 /*
 if args.runtests {
  runtests();
  exit(0);
 }
 */

 /*
 if true == args.help2 { 
  println!( "filefinder -x help");
  println!( "filefinder -x '(help)'");
  return;
 }
 */

 let path = PathBuf::from( args.path.as_ref().unwrap_or_else( || { eprintln!("--path expected"); exit( 1);}));

 let mut interpreter = Interpreter::new();

 if None == args.path { panic!("path has to be set");}

 if ! args.check_expression.is_empty() {
  
  let path = PathBuf::from( args.path.unwrap());

  if args.check_expression.clone().iter() // 3y18vmwgej // TODO : into function
   .map( | exp | sexp::parse( exp.as_str()).unwrap())
   .map( | stmt | interpreter.interpret_term( State{ path: Some( &path ), stmt: stmt}))
   .fold( true, | accu, res | accu && res) {
   println!("true");
  }
  else
  {
   println!("false");
  }
 }
 else 
 {

  let mut tree_walk = treewalk::TreeWalk::new( path);

  {
   args.exclude_from_file.clone().iter()
    .map( |fname| {
     // let f = File::open( fname).unwrap_or_else( || { eprintln!("cannot open file: ''{}''", fname); exit( 1);});
     let s = read_to_string( fname).unwrap_or_else( | _ | { eprintln!("cannot open file: ''{}''", fname); exit( 1);});
     let excluded_filenames = s.split( | c | { c == '\n' || c == '\r'} );
     excluded_filenames.fold( 0, | _a, k | { // TODO : fold ?
       tree_walk.insert_excluded_filename( PathBuf::from( k.trim()));
       0
      }
     )
    })
    .fold( 0, | _a, _k| 0);
  }

  loop {

   match tree_walk.next() {

    None => break,

    Some( path) => {

     if args.expression.clone().iter() // 3y18vmwgej 
      .map( | exp | sexp::parse( exp.as_str()).unwrap())
      .map( | stmt | interpreter.interpret_term( State{ path: Some( &path ), stmt: stmt}))
      .fold( true, | accu, res | accu && res) {
      println!("{}", path.display());
     }

    },

   }

   interpreter.tree_walk_methods.transmit( &mut tree_walk);
  }
 }

 // println!("{:?}", args2);
}
