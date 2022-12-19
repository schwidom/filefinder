
extern crate clap;
use clap::Parser;

extern crate sexp;
use sexp::Sexp;
use sexp::Atom;

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

extern crate strfmt;

extern crate chrono;

/*
fn get_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/

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

 #[arg(long)]
 format : Option<String>,

 #[arg(long)]
 follow_symlinks : bool, // TODO : why does a link loop end after ./a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a ?

}

trait SexpOrVec {}

// impl SexpOrVec for Sexp {}
impl SexpOrVec for &[Sexp] {} // slice
impl SexpOrVec for &Sexp {}
// impl SexpOrVec for Vec<Sexp> {}

#[derive(Debug,Clone)]
struct State<'a,T> where T : SexpOrVec {
 // help : bool,
 path : &'a PathBuf,
 stmt : T, // Sexp | Vec<Sexp>
}

#[derive(Default)]
struct TreeWalkMethods{
 cutted : bool,
 injected : VecDeque<PathBuf>,
 already_injected : HashSet<PathBuf>,
}

impl TreeWalkMethods {
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

impl ComparatorTrait<&OsString> for Interpreter {
 fn cmp( &mut self, s1 : &Sexp, s2 : &OsString) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => &OsString::from( value1) == s2,
   Sexp::List( stmt) => self.interpret_cmp_list( &stmt, &s2.to_string_lossy().to_string()),
   _ => panic!(),
  }
 }
}

impl ComparatorTrait<&OsStr> for Interpreter {
 fn cmp( &mut self, s1 : &Sexp, s2 : &OsStr) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => &OsString::from( value1) == s2,
   Sexp::List( stmt) => self.interpret_cmp_list( &stmt, &s2.to_string_lossy().to_string()),
   _ => panic!(),
  }
 }
}

impl ComparatorTrait<&String> for Interpreter {
 fn cmp( &mut self, s1 : &Sexp, s2 : &String) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => value1 == s2,
   Sexp::List( stmt) => self.interpret_cmp_list( &stmt, &s2),
   _ => panic!(),
  }
 }
}

impl ComparatorTrait<u64> for Interpreter {
 fn cmp( &mut self, s1 : &Sexp, u : u64) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::I( value1)) => (*value1 as u64) == u,
   Sexp::List( stmt) => self.interpret_cmp_list_u64( &stmt, u), // TODO
   _ => panic!(),
  }
 }
}

#[derive(Default)]
struct Interpreter {
 tree_walk_methods : TreeWalkMethods,
 regex_map : HashMap<String,regex::Regex>,
}

trait PathBufTrait {
 fn is_empty( &self ) -> bool;
 fn is_readonly( &self ) -> bool;

 // convenience methods
 
 fn cm_size( &self) -> u64;

 fn cm_path( &self) -> String;
 fn cm_basename( &self) -> String;
 fn cm_dirname( &self) -> String;
 fn cm_filestem( &self) -> String;
 fn cm_extension( &self) -> String;
 fn cm_atime( &self) -> String;
 fn cm_ctime( &self) -> String;
 fn cm_mtime( &self) -> String;
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

 fn is_readonly( &self) -> bool {
  match metadata( self) {
   Ok( md) => md.permissions().readonly(),
   _ => false,
  }
 }

 fn cm_size( &self) -> u64 {
  match metadata( self) {
   Ok( md) => md.len(),
   _ => 0,
  }
 }

 fn cm_path( &self) -> String {
  self.to_string_lossy().to_string()
  // self.as_os_str().to_string_lossy().to_string()
 }

 fn cm_basename( &self) -> String {

  if let Some( name) = self.file_name() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
 }

 fn cm_dirname( &self) -> String {

  if let Some( name) = self.parent() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
 }

 fn cm_filestem( &self) -> String {

  if let Some( name) = self.file_stem() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
 }

 fn cm_extension( &self) -> String {

  if let Some( name) = self.extension() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
 }

 fn cm_atime( &self) -> String {
  if let Ok( md) = metadata( self) {
   if let Ok(time) = md.accessed() {
    let dt = chrono::DateTime::<chrono::offset::Local>::from( time);
    return dt.to_string();
   }
  }
  "".to_string()
 }

 fn cm_ctime( &self) -> String {
  if let Ok( md) = metadata( self) {
   if let Ok(time) = md.created() {
    let dt = chrono::DateTime::<chrono::offset::Local>::from( time);
    return dt.to_string();
   }
  }
  "".to_string()
 }

 fn cm_mtime( &self) -> String {
  if let Ok( md) = metadata( self) {
   if let Ok(time) = md.modified() {
    let dt = chrono::DateTime::<chrono::offset::Local>::from( time);
    return dt.to_string();
   }
  }
  "".to_string()
 }

}


impl Interpreter {

 fn new() -> Self { Interpreter::default() }

 fn interpret_cmp_term( &mut self, sexp : &Sexp, subject_str : &String) -> bool {
  match &sexp {
   Sexp::List( stmt) => self.interpret_cmp_list( stmt, subject_str),
   _ => panic!("list expected"),
  }
 }

 fn interpret_string_term_file( &mut self, stmt : &[Sexp]) -> String {
  if 2 != stmt.len() { panic!("string command file follows fieldname and filename");}

  let file = match &stmt[1] {
   Sexp::Atom( Atom::S( filename)) => PathBuf::from( filename),
   _ => panic!("filename must be string"),
  };

  if let Sexp::Atom( Atom::S( command)) = &stmt[0] {
   match command.as_str() {
    "atime" => file.cm_atime(),
    "mtime" => file.cm_mtime(),
    "ctime" => file.cm_ctime(),
    "size_string" => file.cm_size().to_string(),
    _ => panic!( "string command fieldname not implemented {}", command),
   }
  } else {
   panic!("string command fieldname must be string")
  }
  
 }

 fn interpret_string_term( &mut self, stmt : &[Sexp]) -> String {
  
  if 0 == stmt.len() { panic!("string command must be nonempty list");}

  let command = match &stmt[0] {
   Sexp::Atom( Atom::S( command)) => command,
   _ => panic!("string command not found {:?}", &stmt[0]),
  };

  match command.as_str() {
   "file2" => self.interpret_string_term_file( &stmt[1..]),
   _ => panic!("string command not found {}", command),
  }
 }

 fn interpret_cmp_list( &mut self, stmt : &[Sexp], subject_str : &String) -> bool {

  if 0 == stmt.len() { return true;}

  // TODO : check if it is a number

  if let Sexp::Atom( Atom::S( command)) = &stmt[0] {

   if let Some( res) = match command.as_str() {
    "and0" => Some( stmt[1..].iter().fold( true, | accu, value | accu && self.interpret_cmp_term( &value, &subject_str))),
    "or0" => Some( stmt[1..].iter().fold( false, | accu, value | accu || self.interpret_cmp_term( &value, &subject_str))),
    "not0" => Some( ! self.interpret_cmp_list( &stmt[1..], subject_str)),
    _ => None,
   } { return res;}

   if 1 == stmt.len() { panic!("no parameter to command {}", &stmt[0])}

   let parameter_tmp : String;

   let parameter = match &stmt[1] {
    Sexp::Atom( Atom::S( parameter)) => parameter,
    Sexp::List( sexp) => { parameter_tmp = self.interpret_string_term( sexp); &parameter_tmp },
    _ => panic!( "1433y10cek"),
   };

   {

    return match command.as_str() {
     "regex1" => {
      if ! self.regex_map.contains_key( parameter) { 
       self.regex_map.insert( parameter.clone(), regex::Regex::new(parameter.as_str()).unwrap());
      }

      let regex = &self.regex_map[parameter]; // copy

      return regex.is_match( subject_str.as_str());
     },
     "startswith1" => { subject_str.starts_with( parameter)},
     "endswith1" => { subject_str.ends_with( parameter)},
     "contains1" => { subject_str.find( parameter) != None},
     "<1" => { subject_str < parameter},
     ">1" => { subject_str > parameter},
     "<=1" => { subject_str <= parameter},
     ">=1" => { subject_str >= parameter},
     "=1" => { subject_str == parameter},
     _ => panic!("unknown comparison operator {}", command),
    } && self.interpret_cmp_list( &stmt[2..], &subject_str);
   }  
  }
  panic!("did not match {:?}", &stmt[0]) // e.g. when it is a number
 }

 fn interpret_cmp_term_u64( &mut self, sexp : &Sexp, subject_u64 : u64) -> bool {
  match &sexp {
   Sexp::List( stmt) => self.interpret_cmp_list_u64( stmt, subject_u64),
   _ => panic!("list expected"),
  }
 }

 fn interpret_cmp_list_u64( &mut self, stmt : &[Sexp], subject_u64 : u64) -> bool {

  if 0 == stmt.len() { return true;}

  // TODO : check if it is a number

  if let Sexp::Atom( Atom::S( command)) = &stmt[0] {

   if let Some( res) = match command.as_str() {
    "and0" => Some( stmt[1..].iter().fold( true, | accu, value | accu && self.interpret_cmp_term_u64( &value, subject_u64))),
    "or0" => Some( stmt[1..].iter().fold( false, | accu, value | accu || self.interpret_cmp_term_u64( &value, subject_u64))),
    "not0" => Some( ! self.interpret_cmp_list_u64( &stmt[1..], subject_u64)),
    _ => None,
   } { return res;}

   if 1 == stmt.len() { panic!("no parameter to command {}", &stmt[0])}

   let parameter = match &stmt[1] {
    Sexp::Atom( Atom::I( parameter)) => *parameter as u64,
    Sexp::Atom( Atom::F( parameter)) => *parameter as u64,
    _ => panic!("wrong type: {:?} {:?}", &stmt[0], &stmt[1]),
   };

   return match command.as_str() {
    "<1" => { subject_u64 < parameter},
    ">1" => { subject_u64 > parameter},
    "<=1" => { subject_u64 <= parameter},
    ">=1" => { subject_u64 >= parameter},
    "=1" => { subject_u64 == parameter},
    _ => panic!("unknown comparison operator {}", command),
   } && self.interpret_cmp_list_u64( &stmt[2..], subject_u64);
  }
  panic!("did not match {:?} {:?}", &stmt[0], &stmt[1]) // e.g. when it is another type
 }

 fn interpret_term( &mut self, state : &State<&Sexp>) -> bool {

  match &state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "t" => true,
     "f" => false,
     "cut" => { self.tree_walk_methods.cut(); true},
     "uncut" => { self.tree_walk_methods.uncut(); true},
     "inject" => { self.tree_walk_methods.inject(state.path); true},
     "isdir" => state.path.is_dir(),
     "isfile" => state.path.is_file(),
     "islink" => state.path.is_symlink(),
     "exists" => state.path.exists(),
     "isempty" => state.path.is_empty(),
     "isreadonly" => state.path.is_readonly(),
     _ => panic!( "{}", "not implemented as value/command : ".to_string() + atom),
   }},

   Sexp::List( list) => {
    self.interpret_slice( &State::<&[Sexp]>{ stmt : &list[..], path : state.path })
   },

   _ => panic!("not implemented bc03co4trb")
  }
 }

 fn cont2( &mut self, i : usize, state : &State<&[Sexp]>) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : state.path })
 }

 fn cont3( &mut self, i : usize, state : &State<&[Sexp]>, path : &PathBuf) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : path })
 }

 fn interpret_slice( &mut self, state : &State<&[Sexp]>) -> bool {
  if state.stmt.is_empty() { return true;}

  let mut cont = | i : usize | -> bool { 
   self.interpret_slice( &State::<&[Sexp]>{ stmt : &state.stmt[ i..] , path : state.path }) 
  };

  if let Sexp::Atom(Atom::S( atom)) = &state.stmt[0] {
   match atom.as_str() {
    // "help" => true, // TODO
    "ct0" => true, // comment true
    "cf0" => false, // comment false
    "t0" => { cont( 1); true },
    "f0" => { cont( 1); false },
    "or0" => state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term( &State::<&Sexp>{ stmt : &k, path : state.path })
    ),
    "and0" => state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term( &State::<&Sexp>{ stmt : &k, path : state.path })
    ),
    "progn0" => state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term( &State::<&Sexp>{ stmt : k, path : state.path })
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
        let mut newpath = state.path.clone();
        newpath.push(PathBuf::from(path));
        self.cont3( 2, &state, &newpath)
       },
       Sexp::List( stmt) => {
        let mut res : bool = false;
        if let Ok( direntries) = state.path.read_dir() { 
         for direntry in direntries {
          let path = direntry.unwrap().path();
          if self.interpret2( stmt, &path) 
          {
           res = self.cont3( 2, &state, &path);
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
    },
    "inback0" => {
      let mut newpath = state.path.clone();
      newpath.pop();
      self.cont3( 1, &state, &newpath)
    },
    "dirname1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_dirname()) &&
      self.cont2( 2, &state)
    }, 
    "path1" => { 
       self.cmp( &state.stmt[1], &state.path.cm_path()) &&
       self.cont2( 2, &state)
    }, 
    "basename1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_basename()) &&
      self.cont2( 2, &state)
    }, 
    "filestem1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_filestem()) &&
      self.cont2( 2, &state)
    }, 
    "extension1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_extension()) &&
      self.cont2( 2, &state)
    }, 
    "atime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_atime()) &&
      self.cont2( 2, &state)
    }, 
    "ctime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_ctime()) &&
      self.cont2( 2, &state)
    }, 
    "mtime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_mtime()) &&
      self.cont2( 2, &state)
    }, 
    "size_string1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_size().to_string()) &&
      self.cont2( 2, &state)
    }, 
    "size1" => { 
      self.cmp( &state.stmt[1], state.path.cm_size()) &&
      self.cont2( 2, &state)
    }, 
    "isdir0" => { state.path.is_dir() && cont( 1) },
    "isfile0" => { state.path.is_file() && cont( 1) },
    "islink0" => { state.path.is_symlink() && cont( 1) },
    "exists0" => { state.path.exists() && cont( 1) },
    "isempty0" => { state.path.is_empty() && cont( 1) },
    "isreadonly0" => { state.path.is_readonly() && cont( 1) },
    _ => panic!("not implemented as command : ''{}''", atom),
   }
  } else {
   panic!("string expected ''{}''", &state.stmt[0])
  }
 }

 fn interpret( &mut self, v : &Vec<String>, path : &PathBuf) -> bool {
  v.iter() 
   .map( | exp | sexp::parse( exp.as_str()).unwrap())
   .map( | stmt | self.interpret_term( &State{ path: &path, stmt: &stmt}))
   .fold( true, | accu, res | accu && res)
 }

 fn interpret2( &mut self, stmt : &[Sexp], path : &PathBuf) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ path: &path, stmt: stmt}) // TODO : State{ stmt : &T}
 }

}

fn create_hash_set_from_excluded_files( args : &Args) -> HashSet<PathBuf> {

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

fn path_format( path : & PathBuf, re : &regex::Regex, format : &String) -> String {
 let mut ht : HashMap<String,String> = HashMap::new();
 for mat in re.find_iter( format) {
  let s = mat.as_str();
  let symbol = &s[1..s.len()-1];
  
  fn bool_to_string( b : bool) -> String {
   if b { "true".to_string() } else { "false".to_string() } 
  }

  match symbol {
   "size" => { ht.insert( "size".to_string(), path.cm_size().to_string());},
   "path" => { ht.insert( "path".to_string(), path.cm_path());},
   "basename" => { ht.insert( "basename".to_string(), path.cm_basename());},
   "dirname" => { ht.insert( "dirname".to_string(), path.cm_dirname());},
   "filestem" => { ht.insert( "filestem".to_string(), path.cm_filestem());},
   "extension" => { ht.insert( "extension".to_string(), path.cm_extension());},
   "isdir" => {      ht.insert( "isdir".to_string(), bool_to_string( path.is_dir()));},
   "isfile" => {     ht.insert( "isfile".to_string(), bool_to_string( path.is_file()));},
   "islink" => {     ht.insert( "islink".to_string(), bool_to_string( path.is_symlink()));},
   "exists" => {     ht.insert( "exists".to_string(), bool_to_string( path.exists()));},
   "isempty" => {    ht.insert( "isempty".to_string(), bool_to_string( path.is_empty()));},
   "isreadonly" => { ht.insert( "isreadonly".to_string(), bool_to_string( path.is_readonly()));},
   "atime" => { ht.insert( "atime".to_string(), path.cm_atime());},
   "ctime" => { ht.insert( "ctime".to_string(), path.cm_ctime());},
   "mtime" => { ht.insert( "mtime".to_string(), path.cm_mtime());},

   _ => (),
  }
 }
 strfmt::strfmt( &format.as_str(), &ht).unwrap()
}

fn main() {
 
 let re = regex::Regex::new( r#"\{[^{]+\}"#).unwrap();

 let args = Args::parse();

 let mut interpreter = Interpreter::new(); // vr9e9deprc 

 if args.files_from_stdin {

  use std::io::stdin;

  let mut input_line = String::new();

  let excluded_files = create_hash_set_from_excluded_files( &args);

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
   

   if interpreter.interpret( &args.expression, &path)
   {
    if let Some( format) = &args.format {
     println!("{}", path_format( &path, &re, &format));
    } else {
     println!("{}", path.display());
    }
   }

   input_line.clear();
  }

  exit( 0);
 }
 
 let path = PathBuf::from( args.path.as_ref().unwrap_or_else( || { eprintln!("fatal: --path expected"); exit( 1);}));

 if None == args.path { panic!("fatal: path has to be set");}

 if args.check_expression {
  
  let path = PathBuf::from( args.path.unwrap());

  if interpreter.interpret( &args.expression, &path) {
   print!("true");
  }
  else
  {
   print!("false");
  }

  if let Some( format) = &args.format {
   println!(" {}", path_format( &path, &re, &format));
  } else {
   println!();
  }

  exit( 0);
 }
 
 {

  let mut tree_walk = treewalk::TreeWalk::new( path);
 
  tree_walk.follow_symlinks = args.follow_symlinks;

  if let &Some( ref cut_log) = &args.debug_log_cuts_file {
   tree_walk.cut_log = Some( LineWriter::new( File::create( cut_log).unwrap()));
  }

  {
    tree_walk.excluded_files = create_hash_set_from_excluded_files( &args);
  }

  loop {

   match tree_walk.next() {

    None => break,

    Some( path) => {

     if interpreter.interpret( &args.expression, &path) {
 
      if let Some( format) = &args.format {
       println!("{}", path_format( &path, &re, &format));
      } else {
       println!("{}", path.display());
      }
     }

    },

   }

   interpreter.tree_walk_methods.transmit( &mut tree_walk);
  }
 }
}
