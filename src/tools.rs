
#![allow(unused)]

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum AO {
 And,
 Or,
}

// TODO ao mit variablen parameterlisten
#[macro_export] // exportiert nach crate::ao
macro_rules! ao {
 ( $lhs:expr, $a:expr, $rhs:expr) => 
  ( match $a { AO::And => { $lhs && $rhs }, AO::Or => { $lhs || $rhs },} );
 ( $a:expr ) => ( match $a { AO::And => true, AO::Or => false,} );
}
