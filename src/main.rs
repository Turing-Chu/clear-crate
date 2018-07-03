extern crate semver;

// std
use std::path::Path;
use std::collections::HashMap;
use std::process;
use std::fs::remove_dir_all;
use std::io::Result;

// third
use semver::Version;


fn main() -> Result<()>{
	// check path exists or not
	const BASE_DIR :&str = "/usr/local/apps/cargo/registry/src/github.com-1ecc6299db9ec823";
	let clear_path = Path::new(&BASE_DIR);
	if !(clear_path.exists() || clear_path.is_dir()){
		eprintln!("Error: No such directory {}", clear_path.display());
		process::exit(1);
	}

	// get crate vith version number lists
	let mut modules_v: Vec<String> = Vec::new();
	for entry in clear_path.read_dir().expect("read_dir call failed") {
		if let Ok(entry) = entry {
			modules_v.push(entry.path().file_name().unwrap().to_str().unwrap().to_string());
		}
	}

	// no effective , can be removed
    modules_v.sort_by(|a, b| b.cmp(a));  // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by

	// get crate version lists
	let mut modules: HashMap<String, Vec<String>> = HashMap::new();
	for m in modules_v {
		let module = m.get(0..m.rfind('-').unwrap()).unwrap().to_string();
		let version = m.get(1+m.rfind('-').unwrap()..).unwrap().to_string();
		if modules.contains_key(&module) {
			modules.get_mut(&module).unwrap().push(version);
		} else {
			modules.insert(module, vec![version]);
		}
	}
	
	// remove old version
	for (module, versions) in modules.iter() {
		if 1 == versions.len() {
			continue;
		}
		let mut it = versions.iter();
		let saved_version = it.next();
		loop {
			let v = it.next();
			if None == v {
				break;
			}
			if Version::parse(&v.unwrap()) < Version::parse(&saved_version.unwrap()) {
				let del_path = String::from(BASE_DIR) + "/" + module + "-" + v.unwrap();
				println!("remove {:?}", Path::new(&del_path));
				remove_dir_all(Path::new(&del_path))?;
			} else {
				let del_path = String::from(BASE_DIR) + "/" + module + "-" + saved_version.unwrap();
				println!("remove {:?}", Path::new(&del_path));
				remove_dir_all(Path::new(&del_path))?;
			}
		}
	}
	Ok(())
}
