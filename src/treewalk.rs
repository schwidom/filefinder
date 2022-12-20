
use std::path::PathBuf;
use std::fs::read_dir;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::io::LineWriter;
use std::fs::File;
use std::io::Write; // write_fmt

pub struct TreeWalk
{
 path : VecDeque<PathBuf>,
 next_dir : Option<PathBuf>,
 pub excluded_files : HashSet<PathBuf>, // darf nicht mit ./ beginnen
 pub cut_log : Option<LineWriter<File>>,
 pub follow_symlinks : bool,
 followed_link : HashSet<PathBuf>, // per canonicalize
}

impl TreeWalk {
 #[allow(unused)]
 pub fn new( path : PathBuf) -> Self { 
  TreeWalk{ 
   path : VecDeque::from([path]),
   next_dir : None, 
   excluded_files : HashSet::new(), 
  cut_log : None, 
  follow_symlinks: false,
  followed_link : HashSet::new(),
 }}
 pub fn cut( &mut self) { 
  if let &mut Some( ref mut cut_log) = &mut self.cut_log {
   match cut_log.write_fmt( format_args!( "cut: {:?}\n", self.next_dir)) {
    Err(e) => panic!("{:?}", e),
    _ => (),
   }
  }
  self.next_dir = None;
 }
 pub fn inject( &mut self, path : PathBuf) {
  self.path.push_back( path);
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

  self.next_dir = None;

  let ret = self.path.pop_front();

  if let Some( path) = &ret {
   // assert!( !( path.is_dir() &&  path.is_symlink())); // happens

   loop {
 
    if ! path.is_dir() { break; }

    if ! path.is_symlink() {
     self.next_dir = Some( path.clone());
     break;
    }

    if ! self.follow_symlinks { break; }

    if let Ok( path_canonical) = path.canonicalize() {
     if ! self.followed_link.contains(&path_canonical) {
      self.followed_link.insert(path_canonical);
      self.next_dir = Some( path.clone());
     }
    } else {
     self.next_dir = Some( path.clone());
    }

    break;
   }

  }

  ret
 }
}

