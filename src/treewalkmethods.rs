use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::PathBuf;

// use filefinder::treewalk; // lib.rs // unresolved import `filefinder` // TODO : ???
use crate::treewalk; // lib.rs 

#[derive(Default)]
pub struct TreeWalkMethods{
 cutted : bool,
 injected : VecDeque<PathBuf>,
 already_injected : HashSet<PathBuf>,
}

impl TreeWalkMethods {
 pub fn cut( &mut self) { self.cutted = true;}
 pub fn uncut( &mut self) { self.cutted = false;}
 pub fn inject( &mut self, path : &PathBuf) { self.injected.push_back( path.clone());}
 pub fn injectonce( &mut self, path : &PathBuf) {
  if ! self.already_injected.contains(path) {
   self.already_injected.insert( path.clone());
   self.injected.push_back( path.clone());
  }
 }
 pub fn transmit( &mut self, tree_walk : &mut treewalk::TreeWalk) {
  if self.cutted { tree_walk.cut(); }
  self.cutted = false; // fix (wichtig)
  while let Some( x) = self.injected.pop_front() {
   tree_walk.inject(x);
  }
 }
}
