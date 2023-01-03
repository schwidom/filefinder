use std::path::PathBuf;

use std::fs::metadata;

pub trait PathBufTrait {
 fn is_empty( &self ) -> bool;
 fn is_readonly( &self ) -> bool;

 // convenience methods
 
 fn cm_size( &self) -> u64;

 fn cm_path( &self) -> String;
 fn cm_realpath( &self) -> String;
 fn cm_readlink( &self) -> String;
 fn cm_basename( &self) -> String;
 fn cm_dirname( &self) -> String;
 fn cm_filestem( &self) -> String;
 fn cm_extension( &self) -> String;
 fn cm_atime( &self) -> String;
 fn cm_ctime( &self) -> String;
 fn cm_mtime( &self) -> String;

 fn cm_len( &self) -> u64;
 fn cm_depth( &self) -> u64;
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

 fn cm_realpath( &self) -> String {

  if let Ok( name) = self.canonicalize() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
 }

 fn cm_readlink( &self) -> String {

  if let Ok( name) = self.read_link() {
   return name.to_string_lossy().to_string();
  }

  return "".to_string();
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

 fn cm_len( &self) -> u64 {
  self.to_string_lossy().len() as u64
 }

 fn cm_depth( &self) -> u64 {
  self.ancestors().count() as u64
 }

}


