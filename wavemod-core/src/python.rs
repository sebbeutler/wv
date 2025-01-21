#![allow(unused)]

use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyNone, PyTuple};
use pyo3_ffi::c_str;

#[pyfunction]
fn pytest() {
	println!("wavemod_rs.test");
}

pub fn execute_py(script_name: &str, py_code: &str) -> PyResult<()> {
	Python::with_gil(|py| {
		// Load sys module
		let sys = py.import("sys")?;
		let sys_modules = sys.getattr("modules")?;
		let path: Bound<'_, PyList> = sys.getattr("path")?.downcast_into()?;

		// Load wavemod_rs module
		let wavemod_rs = PyModule::new(py, "wavemod_rs")?;
		wavemod_rs.add_function(wrap_pyfunction!(pytest, &wavemod_rs)?)?;
		sys_modules.set_item("wavemod_rs", wavemod_rs)?;

		// Load wavemod_py path
		let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
		let wavemod_py_folder = crate_root
			.join("src-py/")
			.canonicalize()
			.expect("Cannot find wavemod_py folder");
		path.insert(0, wavemod_py_folder.to_str().unwrap())?;

		// Load wavemod_plugin code
		let script_name_cstr = std::ffi::CString::new(script_name).unwrap();
		let py_code_cstr =
			std::ffi::CString::new(format!("import wavemod as wmd\n\n{}", py_code)).unwrap();
		let wavemod_plugin = PyModule::from_code(
			py,
			&py_code_cstr,
			&script_name_cstr,
			c_str!("wavemod_plugins"),
		)?;

		// Call wavemod_plugin built-in
		let func = wavemod_plugin.getattr("build")?;
		func.call0()?.downcast::<PyNone>()?;

		// Call pip package example
		let numpy = PyModule::import(py, "numpy")?.getattr("array")?;
		let array = numpy.call1((PyTuple::new(py, &[1, 2, 3, 4])?,))?;
		println!("User's numpy array: {}", array);

		Ok(())
	})
}

#[test]
fn test_python() {
	let result = execute_py(
		"plug1",
		r#"

# WMD is default imported
# import wavemod as wmd

wmd.tests._test()
print(wmd.CURRENT_NODE)

def build():
    print("Request py build")

"#,
	);

	assert!(result.is_ok(), "execute_py failed: {:?}", result.err());
}
