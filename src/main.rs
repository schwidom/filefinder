
extern crate clap;
use clap::Parser;

use std::path::PathBuf;

use filefinder::pathbuf::PathBufTrait; // has to be visible for the PathBuf extensions

use std::process::exit;

use std::collections::HashSet;
use std::collections::HashMap;

use std::fs::File;
use std::fs::read_to_string;

use std::io::LineWriter;

extern crate regex;

extern crate strfmt;

extern crate chrono;

use filefinder::tools::AO; // lib.rs

use filefinder::interpreter::Interpreter; // lib.rs

use filefinder::treewalk; // lib.rs

/*
fn get_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
*/


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
 follow_symlinks : bool, 

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
   "realpath" => { ht.insert( "realpath".to_string(), path.cm_realpath());},
   "readlink" => { ht.insert( "readlink".to_string(), path.cm_readlink());},
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

   "pathlength" => { ht.insert( "pathlength".to_string(), path.cm_len().to_string());},

   "pathdepth" => { ht.insert( "pathdepth".to_string(), path.cm_depth().to_string());},

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
   
   if interpreter.interpret( AO::And, &args.expression, &path)
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

  if interpreter.interpret( AO::And, &args.expression, &path) {
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

  tree_walk.excluded_files = create_hash_set_from_excluded_files( &args);

  loop {

   match tree_walk.next() {

    None => break,

    Some( path) => {

     if interpreter.interpret( AO::And, &args.expression, &path) {
 
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
