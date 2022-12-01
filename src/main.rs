
extern crate clap;
use clap::Parser;

extern crate sexp;
use sexp::Sexp;
use sexp::Atom;

// use std::path::Path;
use std::path::PathBuf;

use std::process::exit;

use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;

use std::ffi::OsString;

use std::fs::File;
use std::fs::read_to_string;


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

 // #[arg(short = 'x', long)]
 // explain : Option<String>,

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

/*
#[derive(Debug)]
enum Return {
 Match( bool),
 Help( String),
}

use Return::*;

*/

  // NOTE : (path ...) und (pathregex ...) wird aussenvorgelassen und über
  // Optionen übergeben (vorerst nur path)  
  // Im Interpreter gibt es immer ein aktuell durch einen TreeWalk ausgewählten Pfad 

/*
  let helpstring =
"filefinder -e (find (name ...) (nameregex ...)
  (command ...) (command ...)
  (or (isfile) (isdir) ...) 
  (in <directory> (isdir) (inregex <dir-regex> (isdir))) ... 
 )
".to_string();
  
  if state.help { 
   Help( 
    match state.stmt {
     Sexp::Atom(Atom::S( atom)) if atom == "help".to_string() => helpstring,
     _ => "".to_string(),
    } )
  } else {
  }
*/

/*
    match list.as_slice() {
                                      // TODO : hashtable
     [Sexp::Atom(Atom::S(atom))] if atom == &"true".to_string() => true,
     [Sexp::Atom(Atom::S(atom))] if atom == &"false".to_string()  => false,
     _ => panic!("not implemented"),
    }
*/

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

struct Interpreter {
 tree_walk_methods : TreeWalkMethods,
}

impl Interpreter {

 fn new() -> Self { Interpreter{ tree_walk_methods : TreeWalkMethods::new() } }

 fn interpret_term( &mut self, state : State<Sexp>) -> bool {

  match &state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "true" => true,
     "false" => false,
     "cut" => { self.tree_walk_methods.cut(); true},
     "uncut" => { self.tree_walk_methods.uncut(); true},
     "inject" => { if let Some( path) = state.path { self.tree_walk_methods.inject(path);} true},
     "isdir" => state.path.unwrap().is_dir(),
     "isfile" => state.path.unwrap().is_file(),
     "islink" => state.path.unwrap().is_symlink(),
     "exists" => state.path.unwrap().exists(),
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
    "ct" => true, // comment true
    "cf" => false, // comment false
    "true" => {
     self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path.clone()});
     true
    },
    "false" => {
     self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path.clone()});
     false
    },
    "or" => state.stmt[1..].iter().fold( false, 
     | i, k | { i || self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path.clone() } ) }
    ),
    "and" => state.stmt[1..].iter().fold( true, 
     | i, k | { i && self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path.clone() } ) }
    ),
    "progn" => state.stmt[1..].iter().fold( true, 
     | _i, k | { self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path.clone() } ) }
    ),
    "not" => { 
      ! self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[1..], path : state.path.clone()})
    }, 
    "cut" => { self.tree_walk_methods.cut(); true},
    "uncut" => { self.tree_walk_methods.uncut(); true},
    "inject" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path));
       true
      } else {
       panic!("inject: path expected") 
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
    "injectonce" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.injectonce( &PathBuf::from( path));
       true
      } else {
       panic!("injectonce: path expected")
      }
    }, 
    "in" => {
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
       newpath.push(PathBuf::from(path));
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 2..], path : Some( &newpath)})
      } else { 
       panic!("string expected") 
      }
    },
    "inback" => {
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
       newpath.pop();
       self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ 1..].to_vec() , path : Some( &newpath)})
      } else { 
       panic!("string expected") 
      }
    },
    "basename" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       // eprintln!("{:?}", path);
       // eprintln!("{:?}", state.path.clone());
       // eprintln!("{:?}", state.path.clone().unwrap().file_name());
       // eprintln!("{:?}", state.path.clone().unwrap().file_name().unwrap());
       // *path == state.path.unwrap().file_name().unwrap().to_str().unwrap().to_string()
       // BUG : path ist ein String während der file_name ein OsString ist, d.h. es gibt kaputte
       // Filenamen, die nicht der UTF-8 Spec entsprechen und damit nicht benannt werden können über die Sexp
       // hier wäre ein encoding regexes o.ä. sinnvoll
       OsString::from( path) == state.path.unwrap().file_name().unwrap()
      } else {
       panic!("string expected")
      }
    }, 
    "filestem" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       OsString::from( path) == state.path.unwrap().file_stem().unwrap()
      } else {
       panic!("string expected")
      }
    }, 
    "extension" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       if let Some( ext) = state.path.unwrap().extension() {
        OsString::from( path) == ext
       } else {
        false
       }
      } else {
       panic!("string expected")
      }
    }, 
    "isdir" => state.path.unwrap().is_dir(),
    "isfile" => state.path.unwrap().is_file(),
    "islink" => state.path.unwrap().is_symlink(),
    "exists" => state.path.unwrap().exists(),
    _ => panic!("not implemented"),
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

 /*
 if let Some( exp) = args.explain {
  let stmt = sexp::parse( exp.as_str()).unwrap();
  println!("help not implemented");
 } else */ {
  
  let path = PathBuf::from( args.path.as_ref().unwrap_or_else( || { eprintln!("--path expected"); exit( 1);}));

  let mut tree_walk = treewalk::TreeWalk::new( path);

  let mut interpreter = Interpreter::new();

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

  if None == args.path { panic!("path has to be set");}

  loop {

   match tree_walk.next() {

    None => break,

    Some( path) => {

     /*
     for exp in args.expression.clone() {
      let stmt = sexp::parse( exp.as_str()).unwrap();
      let res = interpreter.interpret_stmt( State{ path: Some( path.clone() ), stmt: stmt});
      if res { println!("{}", path.display()); }
     }
     */

     if args.expression.clone().iter()
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
