use std::collections::HashMap;

use crate::filesearch::Folders;

pub fn arrange_files(folders : Folders) {
   let mut target_by_date = HashMap::new();

   for target in folders.target {
        target_by_date.insert(target.date, target.path);
   };

}