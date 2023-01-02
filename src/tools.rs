
#![allow(unused)]

#[derive(Debug,Clone,Copy)]
pub enum AO {
 And,
 Or,
}

macro_rules! ao {
 ( $lhs:expr, $a:expr, $rhs:expr) => 
  ( match $a { AO::And => { $lhs && $rhs }, AO::Or => { $lhs || $rhs },} )
}
