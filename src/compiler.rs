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
struct State2<'a, T> where T : SexpOrVec + ?Sized {
 // help : bool,
 defop : AO, 
 stmt : &'a T , // Sexp | [Sexp]
}

trait SexpOrVec {}

// impl SexpOrVec for Sexp {}
// impl SexpOrVec for &[Sexp] {} // slice
// impl SexpOrVec for &Sexp {}
impl SexpOrVec for [Sexp] {} // slice
impl SexpOrVec for Sexp {}
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

struct Function {
 f: Box< dyn FnMut( &mut Compiler, &PathBuf) -> bool >
}

impl Function {
 fn call( &mut self, this : &mut Compiler, path : &PathBuf) -> bool {
  ( self.f ) ( this, path)
 }
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

 fn compile_term( &mut self, state : &State2<Sexp>) ->  Function
 {

  let ret = Function{ f: match state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "t" => Box::new( move | _this, _path | true),
     "f" => Box::new( move | _this, _path | false),

     "cut" => Box::new( move | this, _path | { this.tree_walk_methods.cut(); true}),
     "uncut" => Box::new( move | this, _path | { this.tree_walk_methods.uncut(); true}),
     "inject" => Box::new( move | this, path | { this.tree_walk_methods.inject( path); true}), // TODO : ist das getestet?
     "injectonce" => Box::new( move | this, path | { this.tree_walk_methods.injectonce( path); true}), // TODO : ist das getestet?

     "isdir" => Box::new( move | _this, path | path.is_dir()),
     "isfile" => Box::new( move | _this, path | path.is_file()),
     "islink" => Box::new( move | _this, path | path.is_symlink()),
     "exists" => Box::new( move | _this, path | path.exists()),
     "isempty" => Box::new( move | _this, path | path.is_empty()),
     "isreadonly" => Box::new( move | _this, path | path.is_readonly()),

     _ => panic!( "{}", "not implemented as value/command : ".to_string() + atom),
   }},

   Sexp::List( list) => {
    let list_copy = list.clone();
    // let list_copy = Vec::from(list.clone()); 
    let defop = state.defop;
    Box::new( move | this, path | this.interpret_slice2( &State2::<[Sexp]>{ defop : defop, stmt : &list_copy[..]}, &path) )
   },


   _ => panic!("not implemented bc03co4trb")
  }};

  ret
 }

 fn interpret_term2( &mut self, state : &State2<Sexp>, path : & PathBuf) -> bool {

  match &state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "t" => true,
     "f" => false,
     "cut" => { self.tree_walk_methods.cut(); true},
     "uncut" => { self.tree_walk_methods.uncut(); true},
     "inject" => { self.tree_walk_methods.inject(path); true},
     "isdir" => path.is_dir(),
     "isfile" => path.is_file(),
     "islink" => path.is_symlink(),
     "exists" => path.exists(),
     "isempty" => path.is_empty(),
     "isreadonly" => path.is_readonly(),
     _ => panic!( "{}", "not implemented as value/command : ".to_string() + atom),
   }},

   Sexp::List( list) => {
    self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &list[..]}, &path)
   },

   _ => panic!("not implemented bc03co4trb")
  }
 }

 fn cont2_2( &mut self, i : usize, state : &State2<[Sexp]>, path : &PathBuf) -> bool {
  self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] }, &path)
 }

 fn cont3_2( &mut self, i : usize, state : &State2<[Sexp]>, path : &PathBuf) -> bool {
  self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] }, &path)
 }

 fn cont4_2( &mut self, defop : AO, i : usize, state : &State2<[Sexp]>, path : &PathBuf) -> bool {
  self.interpret_slice2( &State2::<[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] }, &path)
 }

 fn cont4_2c( &mut self, defop : AO, i : usize, state : &State2<[Sexp]>) -> Function {
  // self.interpret_slice2( &State2::<[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] }, &path)
  self.compile_slice( &State2::<[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] })
 }

 // WEITERBEI
 fn compile_slice( &mut self, state : &State2<[Sexp]>) -> Function {

  let defop = state.defop;

  if state.stmt.is_empty() { return Function{ f : Box::new( move | _this, _path | ao!( defop ))};}

  /* // erstmal duch eine der alternativen ersetzen
  let mut cont = | i : usize, path : &PathBuf | -> bool { 
   // self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : path }) 
   self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..]}, &path) 
  };
  */

  let mut cont = | i : usize | -> Function { 
   // self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : path }) 
   // self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..]}, &path) 
   self.compile_slice( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..]}) 
  };

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
 
   let matchresult = Function{ f: match atom.as_str() { // TODO : better name (matchresult)
    // "help" => true, // TODO
    "|0" => { return self.cont4_2c( AO::Or, next_command, &state) },
    "&0" => { return self.cont4_2c( AO::And, next_command, &state) },
    "ct0" => { Box::new( move | _this, _path | true )}, // comment true
    "cf0" => { Box::new( move | _this, _path | false )}, // comment true
    "t0" => { let mut c = cont( 1); return Function{ f: Box::new( move | this, path | { c.call( this, path); true } ) } },
    "f0" => { let mut c = cont( 1); return Function{ f: Box::new( move | this, path | { c.call( this, path); false } ) } },
    "or0" => { return state.stmt[1..].iter().fold( Function{ f: Box::new( move | _this, _path | false ) }, 
     | mut accu , value | {
      let mut value_function = self.compile_term( &State2::<Sexp>{ defop : state.defop, stmt : &value });
      Function{ f: Box::new( move | this, path | accu.call( this, path) || value_function.call( this, path)) }
     }
    )},
    "and0" => { return state.stmt[1..].iter().fold( Function{ f: Box::new( move | _this, _path | true ) }, 
     | mut accu , value | {
      let mut value_function = self.compile_term( &State2::<Sexp>{ defop : state.defop, stmt : &value });
      Function{ f: Box::new( move | this, path | accu.call( this, path) && value_function.call( this, path)) }
     }
    )},
    _ => panic!(),
   }};

  panic!();

   /* // WEITERBEI

   let matchresult = match atom.as_str() { // TODO : better name (matchresult)
    // "help" => true, // TODO
    "|0" => { return self.cont4_2( AO::Or, next_command, &state, &path) },
    "&0" => { return self.cont4_2( AO::And, next_command, &state, &path) },
    "ct0" => { return true}, // comment true
    "cf0" => { return false}, // comment false
    "t0" => { cont( 1); return true },
    "f0" => { cont( 1); return false },
    "or0" => { return state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : &k }, &path)
    )},
    "and0" => { return state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : &k }, &path)
    )},
    "progn0" => { return state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : k }, &path)
    )},
    "not0" => { return ! cont( 1) }, 
    "do0" => { return cont( 1) },
    "cut0" => { 
      self.tree_walk_methods.cut(); 
      return self.cont2_2( 1, &state, &path)
    },
    "uncut0" => {
       self.tree_walk_methods.uncut();
       return self.cont2_2( 1, &state, &path)
    },
    "inject1" => { 
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path2));
       return self.cont2_2( 2, &state, &path)
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
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.injectonce( &PathBuf::from( path2));
       return self.cont2_2( 2, &state, &path)
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {

      match &state.stmt[1] {
       Sexp::Atom( Atom::S(path2)) => { // NOTE : maybe wrong
        let mut newpath = path.clone();
        newpath.push(PathBuf::from(path2));
        return self.cont3_2( 2, &state, &newpath)
       },
       Sexp::List( stmt) => {
        let mut res : bool = false;
        if let Ok( direntries) = path.read_dir() { 
         for direntry in direntries {
          let path = direntry.unwrap().path();
          if self.interpret2( state.defop, stmt, &path) 
          {
           res = self.cont3_2( 2, &state, &path);
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
      let mut newpath = path.clone();
      newpath.pop();
      return self.cont3_2( 1, &state, &newpath)
    },
    "dirname1" => { 
      self.cmp( &state.stmt[1], &path.cm_dirname())
    }, 
    "path1" => { 
      self.cmp( &state.stmt[1], &path.cm_path())
    }, 
    "realpath1" => { 
      self.cmp( &state.stmt[1], &path.cm_realpath()) 
    }, 
    "readlink1" => { 
      self.cmp( &state.stmt[1], &path.cm_readlink())
    }, 
    "basename1" => { 
      self.cmp( &state.stmt[1], &path.cm_basename())
    }, 
    "filestem1" => { 
      self.cmp( &state.stmt[1], &path.cm_filestem())
    }, 
    "extension1" => { 
      self.cmp( &state.stmt[1], &path.cm_extension())
    }, 
    "atime1" => { 
      self.cmp( &state.stmt[1], &path.cm_atime()) 
    }, 
    "ctime1" => { 
      self.cmp( &state.stmt[1], &path.cm_ctime()) 
    }, 
    "mtime1" => { 
      self.cmp( &state.stmt[1], &path.cm_mtime()) 
    }, 
    "size_string1" => { 
      self.cmp( &state.stmt[1], &path.cm_size().to_string()) 
    }, 
    "size1" => { 
      self.cmp( &state.stmt[1], path.cm_size()) 
    }, 
    "pathlength1" => { 
      self.cmp( &state.stmt[1], path.cm_len())
    }, 
    "pathdepth1" => { // TODO : vs. searchdepth
      self.cmp( &state.stmt[1], path.cm_depth()) 
    }, 
    "isdir0" => { path.is_dir() },
    "isfile0" => { path.is_file() },
    "islink0" => { path.is_symlink() },
    "exists0" => { path.exists() },
    "isempty0" => { path.is_empty() },
    "isreadonly0" => { path.is_readonly() },
    "linksto1" => {

     let path = path;

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
 
   // matchresult wird eine Function sein, zurückgegeben wird eine Funktion2, die matchresult und danach self.cont2_2 ruft

   return ao!( matchresult, state.defop, self.cont2_2( next_command, &state, &path))
   */

  } else {
   panic!("string expected ''{}''", &state.stmt[0])
  }
 }

 fn interpret_slice2( &mut self, state : &State2<[Sexp]>, path : &PathBuf) -> bool {
  if state.stmt.is_empty() { return ao!( state.defop ) ;}

  let mut cont = | i : usize | -> bool { 
   // self.interpret_slice( &State::<&[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] , path : path }) 
   self.interpret_slice2( &State2::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..]}, &path) 
  };

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
    "|0" => { return self.cont4_2( AO::Or, next_command, &state, &path) },
    "&0" => { return self.cont4_2( AO::And, next_command, &state, &path) },
    "ct0" => { return true}, // comment true
    "cf0" => { return false}, // comment false
    "t0" => { cont( 1); return true },
    "f0" => { cont( 1); return false },
    "or0" => { return state.stmt[1..].iter().fold( false, 
     | i, k | i || self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : &k }, &path)
    )},
    "and0" => { return state.stmt[1..].iter().fold( true, 
     | i, k | i && self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : &k }, &path)
    )},
    "progn0" => { return state.stmt[1..].iter().fold( true, 
     | _i, k |  self.interpret_term2( &State2::<Sexp>{ defop : state.defop, stmt : k }, &path)
    )},
    "not0" => { return ! cont( 1) }, 
    "do0" => { return cont( 1) },
    "cut0" => { 
      self.tree_walk_methods.cut(); 
      return self.cont2_2( 1, &state, &path)
    },
    "uncut0" => {
       self.tree_walk_methods.uncut();
       return self.cont2_2( 1, &state, &path)
    },
    "inject1" => { 
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.inject(&PathBuf::from( path2));
       return self.cont2_2( 2, &state, &path)
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
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       self.tree_walk_methods.injectonce( &PathBuf::from( path2));
       return self.cont2_2( 2, &state, &path)
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {

      match &state.stmt[1] {
       Sexp::Atom( Atom::S(path2)) => { // NOTE : maybe wrong
        let mut newpath = path.clone();
        newpath.push(PathBuf::from(path2));
        return self.cont3_2( 2, &state, &newpath)
       },
       Sexp::List( stmt) => {
        let mut res : bool = false;
        if let Ok( direntries) = path.read_dir() { 
         for direntry in direntries {
          let path = direntry.unwrap().path();
          if self.interpret2( state.defop, stmt, &path) 
          {
           res = self.cont3_2( 2, &state, &path);
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
      let mut newpath = path.clone();
      newpath.pop();
      return self.cont3_2( 1, &state, &newpath)
    },
    "dirname1" => { 
      self.cmp( &state.stmt[1], &path.cm_dirname())
    }, 
    "path1" => { 
      self.cmp( &state.stmt[1], &path.cm_path())
    }, 
    "realpath1" => { 
      self.cmp( &state.stmt[1], &path.cm_realpath()) 
    }, 
    "readlink1" => { 
      self.cmp( &state.stmt[1], &path.cm_readlink())
    }, 
    "basename1" => { 
      self.cmp( &state.stmt[1], &path.cm_basename())
    }, 
    "filestem1" => { 
      self.cmp( &state.stmt[1], &path.cm_filestem())
    }, 
    "extension1" => { 
      self.cmp( &state.stmt[1], &path.cm_extension())
    }, 
    "atime1" => { 
      self.cmp( &state.stmt[1], &path.cm_atime()) 
    }, 
    "ctime1" => { 
      self.cmp( &state.stmt[1], &path.cm_ctime()) 
    }, 
    "mtime1" => { 
      self.cmp( &state.stmt[1], &path.cm_mtime()) 
    }, 
    "size_string1" => { 
      self.cmp( &state.stmt[1], &path.cm_size().to_string()) 
    }, 
    "size1" => { 
      self.cmp( &state.stmt[1], path.cm_size()) 
    }, 
    "pathlength1" => { 
      self.cmp( &state.stmt[1], path.cm_len())
    }, 
    "pathdepth1" => { // TODO : vs. searchdepth
      self.cmp( &state.stmt[1], path.cm_depth()) 
    }, 
    "isdir0" => { path.is_dir() },
    "isfile0" => { path.is_file() },
    "islink0" => { path.is_symlink() },
    "exists0" => { path.exists() },
    "isempty0" => { path.is_empty() },
    "isreadonly0" => { path.is_readonly() },
    "linksto1" => {

     let path = path;

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
 
   return ao!( matchresult, state.defop, self.cont2_2( next_command, &state, &path))

  } else {
   panic!("string expected ''{}''", &state.stmt[0])
  }
 }

 fn interpret2( &mut self, defop : AO, stmt : &[Sexp], path : &PathBuf) -> bool {
  self.interpret_slice2( &State2::<[Sexp]>{ defop : defop, stmt: stmt}, &path) // TODO : State{ stmt : &T}
 }
}

impl Compiler {

 fn compile2( & mut self, defop : AO, v : &Vec<String>) -> Function
 {

 let expressions : Vec<_> = v.iter().map( | exp | sexp::parse( exp.as_str()).unwrap()).collect();
 let states : Vec<_> = expressions.iter().map( | stmt | State2{ defop : defop, stmt: stmt}).collect();

 let mut v2 : Vec<Function> = states.iter().map( | state | self.compile_term( state)).collect();
 // let mut v2 : Vec<Function> = states.into_iter().map( move | state | { self.compile_term( &state)} ).collect();

  Function{ f : Box::new ( move | this, path | {
   v2.iter_mut() 
    .map( | f : &mut Function | f.call( this, path))
    .fold( true, | accu , res | accu && res)
  })}
 }

 pub fn compile( & mut self, defop : AO, v : & Vec<String>) -> impl FnMut( &mut Self, &PathBuf) -> bool
 {
  let mut res = self.compile2( defop, &v);
  move | this, path | res.call( this, path)
 }

 // pub fn compile<'a>( & mut self, defop : AO, v : &'a Vec<String>) -> impl FnMut( &mut Self, &PathBuf) -> bool + 'a {
 pub fn compile_unused<'a>( & mut self, defop : AO, v : &'a Vec<String>) -> impl FnMut( &mut Self, &PathBuf) -> bool + 'a {

  type FT = dyn FnMut( &mut Compiler, &PathBuf) -> bool; 
  type FTB = Box< FT >;

  let mut v2 : Vec<FTB> = v.iter() 
   .map( | exp | sexp::parse( exp.as_str()).unwrap())
   .map( | stmt | { FTB::from( Box::new( move | this : &mut Self, path : &PathBuf | this.interpret_term2( &State2{ defop : defop, stmt: &stmt}, path))) } )
   .collect();

  move | this, path | {
   v2.iter_mut() 
    .map( | f : &mut FTB | f( this, path))
    .fold( true, | accu , res | accu && res)
  }

/*
  move | this, path | {
   v.iter() 
    .map( | exp | sexp::parse( exp.as_str()).unwrap())
    .map( | stmt | this.interpret_term( &State{ defop : defop, path: &path, stmt: &stmt}))
    .fold( true, | accu, res | accu && res)
  }
*/
 }

}
