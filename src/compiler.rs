use std::collections::HashMap;
use std::path::PathBuf;

use crate::pathbuf::PathBufTrait; // has to be visible for the PathBuf extensions

extern crate sexp;
use sexp::Sexp;
use sexp::Atom;

// use filefinder::tools::AO; // lib.rs
use crate::tools::AO; // lib.rs
// #[macro_use]
// use crate::tools::ao; // lib.rs
use crate::ao; // lib.rs
// ao

// use filefinder::treewalkmethods::TreeWalkMethods; // lib.rs
use crate::treewalkmethods::TreeWalkMethods; // lib.rs

#[derive(Debug,Clone)]
struct State<'a,T> where T : SexpOrVec {
 // help : bool,
 defop : AO, 
 path : &'a PathBuf,
 stmt : T, // Sexp | Vec<Sexp>
}

trait SexpOrVec {}

// impl SexpOrVec for Sexp {}
impl SexpOrVec for &[Sexp] {} // slice
impl SexpOrVec for &Sexp {}
// impl SexpOrVec for Vec<Sexp> {}

trait ComparatorTrait<T> {
 fn cmp( &mut self, s1 : &Sexp, s2 : T) -> bool;
}

impl ComparatorTrait<&String> for Compiler {
 fn cmp( &mut self, s1 : &Sexp, s2 : &String) -> bool
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => value1 == s2,
   Sexp::List( stmt) => self.interpret_cmp_list( &stmt, &s2),
   _ => panic!(),
  }
 }
}

impl ComparatorTrait<u64> for Compiler {
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
pub struct Compiler {
 pub tree_walk_methods : TreeWalkMethods,
 regex_map : HashMap<String,regex::Regex>,
}


impl Compiler {

 pub fn new() -> Self { Compiler::default() }

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
    self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &list[..], path : state.path })
   },

   _ => panic!("not implemented bc03co4trb")
  }
 }

 fn cont2( &mut self, i : usize, state : &State<&[Sexp]>) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : state.path })
 }

 fn cont3( &mut self, i : usize, state : &State<&[Sexp]>, path : &PathBuf) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : path })
 }

 fn cont4( &mut self, defop : AO, i : usize, state : &State<&[Sexp]>) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] , path : state.path })
 }

 fn interpret_slice( &mut self, state : &State<&[Sexp]>) -> bool {
  if state.stmt.is_empty() { return ao!( state.defop ) ;}

  let mut cont = | i : usize | -> bool { 
   self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : state.path }) 
  };

/*
  let mut cont4 = | defop : AO, i : usize | -> bool { 
   self.interpret_slice( &State::<&[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] , path : state.path }) 
  };
*/

  if let Sexp::Atom(Atom::S( atom)) = &state.stmt[0] {

   let arity : u8 = {
    let atom_b = atom.as_bytes();
    if atom_b.len() == 0 { panic!("command must be of size >0");}
    else {
     let c : u8 = atom_b[atom_b.len() -1];
     if c < b'0' { panic!("command has to end with digit between 0 - 9 {}", atom); }
     let ret = c - ( b'0');
     if ret > 9 { panic!("command has to end with digit between 0 - 9 {}", atom); }
     ret
    }
   };

   let next_command : usize = 1 + arity as usize;
   
   let matchresult = match atom.as_str() { // TODO : better name (matchresult)
    // "help" => true, // TODO
    "|0" => { return self.cont4( AO::Or, next_command, &state) },
    "&0" => { return self.cont4( AO::And, next_command, &state) },
    "ct0" => { return true}, // comment true
    "cf0" => { return false}, // comment false
    "t0" => { cont( 1); return true },
    "f0" => { cont( 1); return false },
    "or0" => { return state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term( &State::<&Sexp>{ defop : state.defop, stmt : &k, path : state.path })
    )},
    "and0" => { return state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term( &State::<&Sexp>{ defop : state.defop, stmt : &k, path : state.path })
    )},
    "progn0" => { return state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term( &State::<&Sexp>{ defop : state.defop, stmt : k, path : state.path })
    )},
    "not0" => { return ! cont( 1) }, 
    "do0" => { return cont( 1) },
    "cut0" => { 
      self.tree_walk_methods.cut(); 
      return self.cont2( 1, &state)
    },
    "uncut0" => {
       self.tree_walk_methods.uncut();
       return self.cont2( 1, &state)
    },
    "inject1" => { 
      if let Sexp::Atom( Atom::S(path)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path));
       return self.cont2( 2, &state)
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
       return self.cont2( 2, &state)
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {

      match &state.stmt[1] {
       Sexp::Atom( Atom::S(path)) => {
        let mut newpath = state.path.clone();
        newpath.push(PathBuf::from(path));
        return self.cont3( 2, &state, &newpath)
       },
       Sexp::List( stmt) => {
        let mut res : bool = false;
        if let Ok( direntries) = state.path.read_dir() { 
         for direntry in direntries {
          let path = direntry.unwrap().path();
          if self.interpret2( state.defop, stmt, &path) 
          {
           res = self.cont3( 2, &state, &path);
           break;
          }
         }
         return res
        } else { 
         return false
        }

       },
       _ => panic!("error in {}: string or command expected", atom)
      }
    },
    "inback0" => {
      let mut newpath = state.path.clone();
      newpath.pop();
      return self.cont3( 1, &state, &newpath)
    },
    "dirname1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_dirname())
    }, 
    "path1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_path())
    }, 
    "realpath1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_realpath()) 
    }, 
    "readlink1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_readlink())
    }, 
    "basename1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_basename())
    }, 
    "filestem1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_filestem())
    }, 
    "extension1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_extension())
    }, 
    "atime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_atime()) 
    }, 
    "ctime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_ctime()) 
    }, 
    "mtime1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_mtime()) 
    }, 
    "size_string1" => { 
      self.cmp( &state.stmt[1], &state.path.cm_size().to_string()) 
    }, 
    "size1" => { 
      self.cmp( &state.stmt[1], state.path.cm_size()) 
    }, 
    "pathlength1" => { 
      self.cmp( &state.stmt[1], state.path.cm_len())
    }, 
    "pathdepth1" => { // TODO : vs. searchdepth
      self.cmp( &state.stmt[1], state.path.cm_depth()) 
    }, 
    "isdir0" => { state.path.is_dir() },
    "isfile0" => { state.path.is_file() },
    "islink0" => { state.path.is_symlink() },
    "exists0" => { state.path.exists() },
    "isempty0" => { state.path.is_empty() },
    "isreadonly0" => { state.path.is_readonly() },
    "linksto1" => {

     let path = state.path;

     loop {

      if ! path.is_symlink() { break false;}

      let path = path.cm_realpath();

      if path == "" { break false;}

      if let Sexp::Atom( Atom::S( param)) =  &state.stmt[1] {

       let param = PathBuf::from(param).cm_realpath();
  
       if param == "" { break false;}

       break path == param;
      } else { 
       panic!("path expected instead of ''{:?}''", &state.stmt[1]);
      }
     }

    },
    _ => panic!("not implemented as command : ''{}''", atom),
   };
 
   return ao!( matchresult, state.defop, self.cont2( next_command, &state))

  } else {
   panic!("string expected ''{}''", &state.stmt[0])
  }
 }

 fn interpret2( &mut self, defop : AO, stmt : &[Sexp], path : &PathBuf) -> bool {
  self.interpret_slice( &State::<&[Sexp]>{ defop : defop, path: &path, stmt: stmt}) // TODO : State{ stmt : &T}
 }

// TODO
 pub fn compile<'a>( &'a mut self, defop : AO, v : &'a Vec<String>) -> impl FnMut( &PathBuf) -> bool + 'a {

  move | path | {
   v.iter() 
    .map( | exp | sexp::parse( exp.as_str()).unwrap())
    .map( | stmt | self.interpret_term( &State{ defop : defop, path: &path, stmt: &stmt}))
    .fold( true, | accu, res | accu && res)
  }
 }

}
