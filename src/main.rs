
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

use std::fs::File;
use std::fs::read_to_string;

use std::io::LineWriter;
use std::fs::metadata;

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
 check_expression : bool,

 // #[arg(short = 'i', long)]
 // help2 : bool, // optional und ohne Parameter
 
 #[arg(long)]
 exclude_from_file : Vec<String>, // darf nicht mit ./ beginnen

 #[arg(long)]
 debug_log_cuts_file : Option<String>,

 #[arg(long)]
 files_from_stdin : bool,
}

fn runtests() {
 treewalk::tests();
}


trait SexpOrVec {}

impl SexpOrVec for Sexp {}
// impl SexpOrVec for Vec<Sexp> {}
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
 // cut_log : Option<LineWriter<File>>,
}

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

 fn cont2( &mut self, i : usize, state : &State<&[Sexp]>) -> bool {
  self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : state.path })
 }

 fn cont3( &mut self, i : usize, state : &State<&[Sexp]>, path : Option<&PathBuf>) -> bool {
  self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : path })
 }

 fn interpret_slice( &mut self, state : State<&[Sexp]>) -> bool {
  if state.stmt.is_empty() { return true;}

  let mut cont = | i : usize | -> bool { 
   self.interpret_slice( State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : state.path }) 
  };

  if let Sexp::Atom(Atom::S( atom)) = &state.stmt[0] {
   match atom.as_str() {
    // TODO : hashtable
    // "help" => true, // TODO
    "ct0" => true, // comment true
    "cf0" => false, // comment false
    "t0" => { cont( 1); true },
    "f0" => { cont( 1); false },
    "or0" => state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "and0" => state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "progn0" => state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term( State::<Sexp>{ stmt : k.clone(), path : state.path })
    ),
    "not0" => { ! cont( 1) }, 
    "do0" => { cont( 1) },
    "cut0" => { 
      self.tree_walk_methods.cut(); 
      self.cont2( 1, &state)
    },
    "uncut0" => {
       self.tree_walk_methods.uncut();
       self.cont2( 1, &state)
    },
    "inject1" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path));
       self.cont2( 2, &state)
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
       self.cont2( 2, &state)
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {

      match &state.stmt[1] {
       Sexp::Atom( Atom::S(path)) => {
        let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
        newpath.push(PathBuf::from(path));
        self.cont3( 2, &state, Some( &newpath))
       },
       Sexp::List( stmt) => {
        // let mut interpreter = Interpreter::new(); // TODO : brauche ich einen neuen interpreter?
        let mut res : bool = false;
        if let Ok( direntries) = state.path.unwrap().read_dir() { 
         for direntry in direntries {
          let path = direntry.unwrap().path();
          if self.interpret2( stmt, &path) 
          {
           res = self.cont3( 2, &state, Some( &path));
           break;
          }
         }
         res
        } else { 
         false
        }

       },
       _ => panic!("error in {}: string or command expected", atom)
      }

/*
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
       newpath.push(PathBuf::from(path));
       self.cont3( 2, &state, Some( &newpath))
      } else { 
       panic!("error in {}: string expected", atom)
      }
*/
    },
    "inback0" => {
      let mut newpath = state.path.unwrap_or_else( || panic!("no current path given")).clone();
      newpath.pop();
      self.cont3( 1, &state, Some( &newpath))
    },
    "dirname1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().parent().unwrap().as_os_str()) &&
      self.cont2( 2, &state)
    }, 
    "path1" => { 
       self.comparator.cmp( &state.stmt[1], state.path.unwrap().as_os_str()) &&
       self.cont2( 2, &state)
    }, 
    "basename1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().file_name().unwrap()) &&
      self.cont2( 2, &state)
    }, 
    "filestem1" => { 
      self.comparator.cmp( &state.stmt[1], state.path.unwrap().file_stem().unwrap()) &&
      self.cont2( 2, &state)
    }, 
    "extension1" => { 
      if let Some( ext) = state.path.unwrap().extension() {
       self.comparator.cmp( &state.stmt[1], ext) &&
       self.cont2( 2, &state)
      } else {
       false
      }
    }, 
    "isdir0" => { state.path.unwrap().is_dir() && cont( 1) },
    "isfile0" => { state.path.unwrap().is_file() && cont( 1) },
    "islink0" => { state.path.unwrap().is_symlink() && cont( 1) },
    "exists0" => { state.path.unwrap().exists() && cont( 1) },
    "isempty0" => { state.path.unwrap().is_empty() && cont( 1) },
    _ => panic!("not implemented: ''{}''", atom),
   }
  } else {
   panic!("not implemented5 ''{}''", &state.stmt[0])
  }
 }

 fn interpret( &mut self, v : Vec<String>, path : &PathBuf) -> bool {
  v.iter() 
   .map( | exp | sexp::parse( exp.as_str()).unwrap())
   .map( | stmt | self.interpret_term( State{ path: Some( &path ), stmt: stmt}))
   .fold( true, | accu, res | accu && res)
 }

 fn interpret2( &mut self, stmt : &[Sexp], path : &PathBuf) -> bool {
  self.interpret_slice( State::<&[Sexp]>{ path: Some( &path ), stmt: stmt}) // TODO : State{ stmt : &T}
 }

}

fn create_hash_set_from_excluded_files( args : Args) -> HashSet<PathBuf> {

 let mut ret : HashSet<PathBuf> = HashSet::new(); // darf nicht mit ./ beginnen

 args.exclude_from_file.iter()
  .map( |fname| {
   let s = read_to_string( fname).unwrap_or_else( | _ | { eprintln!("cannot open file: ''{}''", fname); exit( 1);});
   let excluded_filenames = s.split( | c | { c == '\n' || c == '\r'} );
   excluded_filenames.for_each( | k | { 
    if k != "" { ret.insert( PathBuf::from( k));}
   });
  })
  .for_each( | _k | () );

 ret
}

fn main() {
 
 let args = Args::parse();

 let mut interpreter = Interpreter::new(); // vr9e9deprc 

 if args.files_from_stdin {

  use std::io::stdin;

  let mut input_line = String::new();

  let excluded_files = create_hash_set_from_excluded_files( args.clone());

  'w:
  while let Ok(nchars) = stdin().read_line(&mut input_line) {

   if 0 == nchars { break } // ^D
   input_line.pop(); // remove \n 

   let path = PathBuf::from(input_line.clone());

   for excluded_file in &excluded_files {
    if path.starts_with( excluded_file) {
     input_line.clear(); // TODO : verbessern
     continue 'w
    }
   }
   

   if interpreter.interpret( args.expression.clone(), &path)
   {
    println!("{}", input_line);
   }

   input_line.clear();
  }

  exit( 0);
 }
 
 let path = PathBuf::from( args.path.as_ref().unwrap_or_else( || { eprintln!("fatal: --path expected"); exit( 1);}));

 if None == args.path { panic!("fatal: path has to be set");}

 if args.check_expression {
  
  let path = PathBuf::from( args.path.unwrap());

  if interpreter.interpret( args.expression.clone(), &path) {
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

  if let &Some( ref cut_log) = &args.debug_log_cuts_file {
   tree_walk.cut_log = Some( LineWriter::new( File::create( cut_log).unwrap()));
  }

  {
    tree_walk.excluded_files = create_hash_set_from_excluded_files( args.clone());
  }

  loop {

   match tree_walk.next() {

    None => break,

    Some( path) => {

     if interpreter.interpret( args.expression.clone(), &path) {
      println!("{}", path.display());
     }

    },

   }

   interpreter.tree_walk_methods.transmit( &mut tree_walk);
  }
 }
}
