#[macro_use]
extern crate cpython;
extern crate noise_search;

use cpython::{Python, PyResult};
use noise_search::index::{Index, Batch, OpenOptions};
use noise_search::json_value::{PrettyPrint};


fn add(_py: Python, index_path: &str, data: &str) -> PyResult<String> {
   let mut batch = Batch::new();
   let options = Some(OpenOptions::Create);
   let mut index = Index::open(&index_path, options).unwrap();
   let result = index.add(data, &mut batch).unwrap();
   index.flush(batch).unwrap();
   Ok(result)
}

fn query(_py: Python, index_path: &str, data: &str) -> PyResult<String> {
   let index = Index::open(index_path, None).unwrap();

   let result = index.query(data, None).unwrap();
   let mut printer = PrettyPrint::new("", "", "");
   let result_strings:  Vec<String> = result.map(|item| {
         let mut buffer: Vec<u8> = Vec::new();
         item.render(&mut buffer, &mut printer).unwrap();
         String::from_utf8(buffer).unwrap()
   }).collect();

   let result_json = &result_strings.join(",");
   Ok(format!("[{}]", result_json))
}

py_module_initializer!(_ruido, init_ruido, PyInit__ruido, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "add", py_fn!(py, add(index_path: &str, data: &str))));
    try!(m.add(py, "query", py_fn!(py, query(index_path: &str, data: &str))));
    Ok(())
});
