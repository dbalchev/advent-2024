use std::{
    env,
    error::Error,
    fs::{self, copy, read_dir},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dest_path = out_dir.join("days.fragment");
    let root_path_str = env::var("CARGO_MANIFEST_DIR")?;
    let root_path = Path::new(&root_path_str);
    let solutions_path = root_path.join("src").join("solutions");
    let mut day_names = Vec::new();
    for dir_entry in read_dir(solutions_path)? {
        let dir_entry = dir_entry?;
        let day_path = dir_entry.path();
        let name = day_path
            .file_stem()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        if name == "mod" {
            continue;
        }
        copy(&day_path, out_dir.join(day_path.file_name().unwrap()))?;
        day_names.push(name);
    }
    let days_content = day_names.join(", ");
    let days_content = ["register_days! {", &days_content, "}\n"].join("");
    fs::write(&dest_path, days_content)?;
    Ok(())
}
