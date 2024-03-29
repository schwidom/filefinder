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

use std::process::Command;

macro_rules! newfunbox{
 ( $value:ident, $inclosure:expr ) => ( Box::new( 
   move | _this, $value | $inclosure
  ));
 ( $this:ident, $value:ident, $inclosure:expr ) => ( Box::new( 
   move | $this, $value | $inclosure
  ));
}

macro_rules! newfunstr {
 ( $closure:expr ) => ( Function2::<String>::new( Box::new( $closure )))
}

macro_rules! newfunstr2 {
 ( $value:ident, $inclosure:expr ) => ( Function2::<String>::new( Box::new( 
   move | _this, $value | $inclosure
  )));
 ( $this:ident, $value:ident, $inclosure:expr ) => ( Function2::<String>::new( Box::new( 
   move | $this, $value | $inclosure
  )));
}

macro_rules! newfunu64 {
 ( $value:ident, $inclosure:expr ) => ( Function2::<u64>::new( Box::new( 
   move | _this, $value | $inclosure
  )));
 ( $this:ident, $value:ident, $inclosure:expr ) => ( Function2::<u64>::new( Box::new( 
   move | $this, $value | $inclosure
  )));
}

macro_rules! newfun{
 ( $value:ident, $inclosure:expr ) => ( Function::new( Box::new( 
   move | _this, $value | $inclosure
  )));
 ( $this:ident, $value:ident, $inclosure:expr ) => ( Function::new( Box::new( 
   move | $this, $value | $inclosure
  )));
}


#[derive(Debug,Clone)]
struct State<'a, T> where T : SexpOrVec + ?Sized {
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

trait ComparatorTraitF<T> {
 fn cmpf( &mut self, s1 : &Sexp) -> Function2<T>;
}

impl ComparatorTraitF<String> for Compiler {
 fn cmpf( &mut self, s1 : &Sexp) -> Function2<String>
 {
  match s1 {
   Sexp::Atom( Atom::S( value1)) => { let value2 = value1.clone(); newfunstr2!( s2, value2 == s2) } ,
   Sexp::List( stmt) => self.compile_cmp_list( &stmt),
   _ => panic!(),
  }
 }
}

trait ComparatorTraitFU64<T> {
 fn cmpfu64( &mut self, s1 : &Sexp) -> Function2<T>;
}

impl ComparatorTraitFU64<u64> for Compiler {
 fn cmpfu64( &mut self, s1 : &Sexp) -> Function2<u64>
 {
  match s1 {
   Sexp::Atom( Atom::I( value1)) => { let value2 = value1.clone() as u64; newfunu64!( s2, value2 == s2) } ,
   Sexp::List( stmt) => self.compile_cmp_list_u64( &stmt),
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
 fn new( f : Box< dyn FnMut( &mut Compiler, &PathBuf) -> bool > ) -> Function {
  Function{ f : f }
 }
 fn call( &mut self, this : &mut Compiler, path : &PathBuf) -> bool {
  ( self.f ) ( this, path)
 }
}

struct Function2<T> {
 f: Box< dyn FnMut( &mut Compiler, T) -> bool >
}

impl<T> Function2<T> {
 fn new( f : Box< dyn FnMut( &mut Compiler, T) -> bool > ) -> Function2<T> {
  Function2::<T>{ f : f }
 }
/*
 fn bx( f : dyn FnMut( &mut Compiler, T) -> bool) -> Function2<T> {
  Function2::<T>::new( Box::new( f))
 }
*/
 fn call( &mut self, this : &mut Compiler, value : T) -> bool {
  ( self.f ) ( this, value)
 }
}


impl Compiler {

 pub fn new() -> Self { Compiler::default() }

 fn compile_cmp_term( &mut self, sexp : &Sexp) -> Function2<String> {
  match &sexp {
   Sexp::List( stmt) => self.compile_cmp_list( stmt),
   _ => panic!("list expected"),
  }
 }

 fn compile_cmp_term_u64( &mut self, sexp : &Sexp) -> Function2<u64> {
  match &sexp {
   Sexp::List( stmt) => self.compile_cmp_list_u64( stmt),
   _ => panic!("list expected"),
  }
 }

 // TODO : compile
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

 // TODO : compile
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

 // subject_str = _object
 fn compile_cmp_list( &mut self, stmt : &[Sexp]) -> Function2<String> {

  if 0 == stmt.len() { return newfunstr!( | _this, _subject_str | true);}

  // TODO : check if it is a number

  if let Sexp::Atom( Atom::S( command)) = &stmt[0] {

   if let Some( res) = match command.as_str() {
    "and0" => Some( stmt[1..].iter().fold( 
      newfunstr2!( _this, _subject_str, true),
      | mut accu, value | {
       let mut f2 = self.compile_cmp_term( value);
       newfunstr2!( this, subject_str, accu.call( this, subject_str.clone()) && f2.call( this, subject_str))
      }
    )),
    "or0" => Some( stmt[1..].iter().fold( 
      newfunstr2!( _this, _subject_str, false),
      | mut accu, value | {
       let mut f2 = self.compile_cmp_term( value);
       newfunstr2!( this, subject_str, accu.call( this, subject_str.clone()) || f2.call( this, subject_str))
      }
    )),
    "not0" => { 
     let mut compiled = self.compile_cmp_list( &stmt[1..]);
     Some( newfunstr2!( this, subject_str, ! compiled.call( this, subject_str)))
    },
    _ => None,
   } { return res;}

   if 1 == stmt.len() { panic!("no parameter to command {}", &stmt[0])}

   let parameter_tmp : String;

   let parameter = match &stmt[1] {
    Sexp::Atom( Atom::S( parameter)) => parameter,
    Sexp::List( sexp) => { parameter_tmp = self.interpret_string_term( sexp); &parameter_tmp },
    _ => panic!( "1433y10cek"),
   }.clone();

   {

    let mut matched = match command.as_str() {
     "regex1" => {
      if ! self.regex_map.contains_key( &parameter) { 
       self.regex_map.insert( parameter.clone(), regex::Regex::new(parameter.as_str()).unwrap());
      }

      // let regex = &self.regex_map[parameter].clone(); // copy // TODO
      let regex = regex::Regex::new(parameter.as_str()).unwrap(); // TODO

      newfunstr!( move | _this, subject_str | regex.is_match( subject_str.as_str()))
     },
     "startswith1" => { newfunstr2!( subject_str, subject_str.starts_with( parameter.as_str()))},
     "endswith1" => { newfunstr2!( subject_str, subject_str.ends_with( parameter.as_str()))},
     "contains1" => {  newfunstr2!( subject_str, subject_str.find( parameter.as_str()) != None)},
     "<1" => { newfunstr2!( subject_str, subject_str < parameter)},
     ">1" => { newfunstr2!( subject_str, subject_str > parameter)},
     "<=1" => { newfunstr2!( subject_str, subject_str <= parameter)},
     ">=1" => { newfunstr2!( subject_str, subject_str >= parameter)},
     "=1" => { newfunstr2!( subject_str, subject_str == parameter)},

     "exec1" => { newfunstr2!( subject_str, 
      { 
       let args = parameter.split_ascii_whitespace().collect::<Vec<_>>();
       let mut child = Command::new( args[0])
        // .args( Vec::from( args[1..]))
        .args( args[1..].to_vec())
        .arg( subject_str).spawn().expect( "os command failed");
       let ecode = child.wait().expect( "failed to wait on child");
       ecode.success()
      })
     },

     _ => panic!("unknown comparison operator {}", command),
    };

    let mut fcont = self.compile_cmp_list( &stmt[2..]);

    // TODO : missing |0, &0 switches
    return newfunstr2!( this, subject_str, matched.call( this, subject_str.clone()) && fcont.call( this, subject_str));

   }
  }
  panic!("did not match {:?}", &stmt[0]) // e.g. when it is a number
 }

 fn compile_cmp_list_u64( &mut self, stmt : &[Sexp]) -> Function2<u64> {

  if 0 == stmt.len() { return newfunu64!( _this, _subject_str, true);}

  // TODO : check if it is a number

  if let Sexp::Atom( Atom::S( command)) = &stmt[0] {

   if let Some( res) = match command.as_str() {
    "and0" => Some( stmt[1..].iter().fold( 
      newfunu64!( _this, _subject_str, true),
      | mut accu, value | {
       let mut f2 = self.compile_cmp_term_u64( value);
       newfunu64!( this, subject_str, accu.call( this, subject_str.clone()) && f2.call( this, subject_str))
      }
    )),
    "or0" => Some( stmt[1..].iter().fold( 
      newfunu64!( _this, _subject_str, false),
      | mut accu, value | {
       let mut f2 = self.compile_cmp_term_u64( value);
       newfunu64!( this, subject_str, accu.call( this, subject_str.clone()) || f2.call( this, subject_str))
      }
    )),
    "not0" => { 
     let mut compiled = self.compile_cmp_list_u64( &stmt[1..]);
     Some( newfunu64!( this, subject_str, ! compiled.call( this, subject_str)))
    },
    _ => None,
   } { return res;}

   if 1 == stmt.len() { panic!("no parameter to command {}", &stmt[0])}

   let parameter = match &stmt[1] {
    Sexp::Atom( Atom::I( parameter)) => *parameter as u64,
    Sexp::Atom( Atom::F( parameter)) => *parameter as u64,
    _ => panic!("wrong type: {:?} {:?}", &stmt[0], &stmt[1]),
   };

   let mut matched = match command.as_str() {
    "<1" => { newfunu64!( subject_u64, subject_u64 < parameter) },
    ">1" => { newfunu64!( subject_u64, subject_u64 > parameter) },
    "<=1" => { newfunu64!( subject_u64, subject_u64 <= parameter) },
    ">=1" => { newfunu64!( subject_u64, subject_u64 >= parameter) },
    "=1" => { newfunu64!( subject_u64, subject_u64 == parameter) },
    _ => panic!("unknown comparison operator {}", command),
   };

   let mut fcont = self.compile_cmp_list_u64( &stmt[2..]);

   return newfunu64!( this, subject_u64, matched.call( this, subject_u64) && fcont.call( this, subject_u64));
  }
  panic!("did not match {:?} {:?}", &stmt[0], &stmt[1]) // e.g. when it is another type
 }

 fn compile_term( &mut self, state : &State<Sexp>) -> Function
 {

  let ret = Function::new( match state.stmt {

   Sexp::Atom(Atom::S( atom)) => {
    match atom.as_str() {
     // "help" => true, // TODO
     "t" => Box::new( move | _this, _path | true),
     "f" => Box::new( move | _this, _path | false),

     "cut" => Box::new( move | this, _path | { this.tree_walk_methods.cut(); true}),
     "uncut" => Box::new( move | this, _path | { this.tree_walk_methods.uncut(); true}),
     "inject" => Box::new( move | this, path | { this.tree_walk_methods.inject( path); true}), // TODO : ist das getestet?
     "injectonce" => Box::new( move | this, path | { this.tree_walk_methods.injectonce( path); true}), // TODO : ist das getestet?, fehlt im Interpreter

     "isdir" => Box::new( move | _this, path | path.is_dir()),
     "isfile" => Box::new( move | _this, path | path.is_file()),
     "islink" => Box::new( move | _this, path | path.is_symlink()),
     "exists" => Box::new( move | _this, path | path.exists()),
     "isempty" => Box::new( move | _this, path | path.is_empty()),
     "isreadonly" => Box::new( move | _this, path | path.is_readonly()),

     _ => panic!( "{}", "not implemented as value/command : ".to_string() + atom),
   }},

   Sexp::List( list) => {
    let mut f2 = self.compile_slice( &State::<[Sexp]>{ defop : state.defop, stmt : &list[..]});
    Box::new( move | this, path | f2.call( this, &path) )
   },

   _ => panic!("not implemented bc03co4trb")
  });

  ret
 }

 fn cont2_2c( &mut self, i : usize, state : &State<[Sexp]>) -> Function {
  self.compile_slice( &State::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] })
 }

 fn cont3_2c( &mut self, i : usize, state : &State<[Sexp]>) -> Function {
  self.compile_slice( &State::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ i..] })
 }

 fn cont4_2c( &mut self, defop : AO, i : usize, state : &State<[Sexp]>) -> Function {
  self.compile_slice( &State::<[Sexp]>{ defop : defop, stmt : &state.stmt[ i..] })
 }

 fn compile_slice( &mut self, state : &State<[Sexp]>) -> Function {

  let defop = state.defop;

  if state.stmt.is_empty() { return newfun!( _path, ao!( defop ));}

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
 
   let mut cont = | | -> Function { 
    self.compile_slice( &State::<[Sexp]>{ defop : state.defop, stmt : &state.stmt[ next_command..]}) 
   };

  // TODO : cont => contf; let mut cont = self.cont2_2c( next_command, &state);

   let mut matchresult = Function::new( match atom.as_str() { // TODO : better name (matchresult)
    // "help" => true, // TODO
    "|0" => { return self.cont4_2c( AO::Or, next_command, &state) },
    "&0" => { return self.cont4_2c( AO::And, next_command, &state) },
    "ct0" => { return newfun!( _path, true )}, // comment true
    "cf0" => { return newfun!( _path, false )}, // comment true
    "t0" => { let mut c = cont(); return newfun!( this, path, { c.call( this, path); true }) },
    "f0" => { let mut c = cont(); return newfun!( this, path, { c.call( this, path); false }) },
    "or0" => { return state.stmt[next_command..].iter().fold( newfun!( _path, false), 
     | mut accu , value | {
      let mut value_function = self.compile_term( &State::<Sexp>{ defop : state.defop, stmt : &value });
      newfun!( this, path, accu.call( this, path) || value_function.call( this, path))
     }
    )},
    "and0" => { return state.stmt[next_command..].iter().fold( newfun!( _path, true), 
     | mut accu , value | {
      let mut value_function = self.compile_term( &State::<Sexp>{ defop : state.defop, stmt : &value });
      newfun!( this, path, accu.call( this, path) && value_function.call( this, path))
     }
    )},
    "progn0" => { return state.stmt[next_command..].iter().fold( newfun!( _path, true), 
     | mut accu , value | {
      let mut value_function = self.compile_term( &State::<Sexp>{ defop : state.defop, stmt : &value });
      newfun!( this, path, { accu.call( this, path); value_function.call( this, path) })
     }
    )},
    "not0" => { let mut c = cont(); return newfun!( this, path, ! c.call( this, path)) },
    "do0" => { return cont() },
    "cut0" => { 
      let mut f2 = self.cont2_2c( next_command, &state);
      return newfun!( this, path, { this.tree_walk_methods.cut();
      f2.call( this, path) })
    },
    "uncut0" => { 
      let mut f2 = self.cont2_2c( next_command, &state);
      return newfun!( this, path, { this.tree_walk_methods.uncut();
      f2.call( this, path) })
    },
    "inject1" => { 
      let mut f2 = self.cont2_2c( next_command, &state);
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       let path3 = path2.clone();
       return newfun!( this, path, { this.tree_walk_methods.inject(&PathBuf::from( path3.clone()));
        f2.call( this, path) })
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
      let mut f2 = self.cont2_2c( next_command, &state); // TODO : use next_command
      if let Sexp::Atom( Atom::S(path2)) = &state.stmt[1] { // TODO : error handling
       let path3 = path2.clone();
       return newfun!( this, path, { this.tree_walk_methods.injectonce(&PathBuf::from( path3.clone()));
        f2.call( this, path) })
      } else {
       panic!("error in {}: string expected", atom)
      }
    }, 
    "in1" => {
      let mut cont3_2 = self.cont3_2c( next_command, &state);
      match &state.stmt[1] {
       Sexp::Atom( Atom::S(path2)) => { // NOTE : maybe wrong
        let path3 = path2.clone();
        return newfun!( this, path, { let mut newpath = path.clone();
          newpath.push(PathBuf::from(path3.clone()));
          cont3_2.call( this, &newpath) })
       },
       Sexp::List( stmt) => {
        let mut compile3c = self.compile3( state.defop, stmt);
        return newfun!( this, path, { let mut res : bool = false;
         if let Ok( direntries) = path.read_dir() { 
          for direntry in direntries {
           let path = direntry.unwrap().path();
           if compile3c.call( this, &path)
           {
            res = cont3_2.call( this, &path);
            break;
           }
          }
          return res
         } else { 
          return false
         } })
       },
       _ => panic!("error in {}: string or command expected", atom)
      }
    },
    "inback0" => {
      let mut f2 = self.cont3_2c( next_command, &state);
      return newfun!( this, path, {
       let mut newpath = path.clone();
       newpath.pop();
       f2.call( this, &newpath)
      })
    },
    "dirname1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_dirname()))
    }, 
    "path1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_path()))
    }, 
    "realpath1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_realpath()))
    }, 
    "readlink1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_readlink()))
    }, 
    "basename1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_basename()))
    }, 
    "filestem1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_filestem()))
    }, 
    "extension1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_extension()))
    }, 
    "atime1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_atime()))
    }, 
    "ctime1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_ctime()))
    }, 
    "mtime1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_mtime()))
    }, 
    "size_string1" => { 
      let mut f2 = self.cmpf( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_size().to_string()))
    }, 
    "size1" => { 
      let mut f2 = self.cmpfu64( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_size()))
    }, 
    "pathlength1" => { 
      let mut f2 = self.cmpfu64( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_len()))
    }, 
    "pathdepth1" => { 
      let mut f2 = self.cmpfu64( &state.stmt[1]); 
      newfunbox!( this, path, f2.call( this, path.cm_depth()))
    }, 
    "filecontents1" => { // TODO: optimize for long files: cmp for filedescriptors
 
      let mut f2 = self.cmpf( &state.stmt[1]);

      newfunbox!( this, path, {

      if ! path.is_file() { false }
      else {
       let contents = std::fs::read_to_string(path.cm_path()).unwrap();
       f2.call( this, contents)
      }
     })
    }, 
    "isdir0" => { newfunbox!( path, path.is_dir()) },
    "isfile0" => { newfunbox!( path, path.is_file()) },
    "islink0" => { newfunbox!( path, path.is_symlink()) },
    "exists0" => { newfunbox!( path, path.exists()) },
    "isempty0" => { newfunbox!( path, path.is_empty()) },
    "isreadonly0" => { newfunbox!( path, path.is_readonly()) },
    "linksto1" => {

     let stmt1 = state.stmt[1].clone(); // TODO : errormessage if no [1] exists

     let param2 = if let Sexp::Atom( Atom::S( param)) = &state.stmt[1] {
      param.clone()
     } else {
        panic!("path expected instead of ''{:?}''", stmt1);
     };

     newfunbox!( path, { 

      loop {

       if ! path.is_symlink() { break false;}

       let path = path.cm_realpath();

       if path == "" { break false;}

       let param = PathBuf::from(param2.clone()).cm_realpath();
  
       if param == "" { break false;}

       break path == param;
      }
     })
    },
    _ => panic!(),
   });

   let mut cont_ret = self.cont2_2c( next_command, &state);

   let defop = state.defop;

   return newfun!( this, path, { 
    ao!( matchresult.call( this, path) , defop, cont_ret.call( this, path))
   });

  } else {
   panic!("string expected ''{}''", &state.stmt[0])
  }
 }

 // TODO : wird das noch gebraucht?
 fn compile3( &mut self, defop : AO, stmt : &[Sexp]) -> Function {
  self.compile_slice( &State::<[Sexp]>{ defop : defop, stmt: stmt})
 }
}

impl Compiler {

 fn compile2( & mut self, defop : AO, v : &Vec<String>) -> Function
 {

  let expressions : Vec<_> = v.iter().map( | exp | sexp::parse( exp.as_str()).unwrap()).collect();
  let states : Vec<_> = expressions.iter().map( | stmt | State{ defop : defop, stmt: stmt}).collect();

  let mut v2 : Vec<Function> = states.iter().map( | state | self.compile_term( state)).collect();

  newfun!( this, path, {
   v2.iter_mut() 
    .map( | f : &mut Function | f.call( this, path))
    .fold( true, | accu , res | accu && res)
  })
 }

 pub fn compile( & mut self, defop : AO, v : & Vec<String>) -> impl FnMut( &mut Self, &PathBuf) -> bool
 {
  let mut res = self.compile2( defop, &v);
  move | this, path | res.call( this, path)
 }
}
