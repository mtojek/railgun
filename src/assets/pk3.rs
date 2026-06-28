use regex::Regex;
use std::{
    io,
    path::{Path, PathBuf},
};

pub fn load_resources(baseq_dir: &Path) {
    let pk3s = list_resources(baseq_dir).unwrap();

    pk3s.iter().for_each(|pk3| {
        println!("{:?}", pk3);
    })
}

fn list_resources(baseq_dir: &Path) -> io::Result<Vec<PathBuf>> {
    println!("baseq3 dir: {}", baseq_dir.display());

    let entries = baseq_dir.read_dir()?;

    let mut pk3s = Vec::new();
    let pk3_regex = Regex::new(r"^pak\d+\.pk3$").unwrap();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            if pk3_regex.is_match(filename) {
                pk3s.push(path);
            }
        }
    }

    pk3s.sort_by_key(|path| {
        path.file_stem() // no extension: pak<N>
            .and_then(|f| f.to_str())
            .and_then(|s| s.strip_prefix("pak"))
            .and_then(|s| s.parse::<u32>().ok())
    });
    return Ok(pk3s);
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn only_pk3_files_sorted() {
        let baseq3 = tempdir().unwrap();

        File::create(baseq3.path().join("pak3.pk3")).unwrap();
        File::create(baseq3.path().join("pak1.pk3")).unwrap();
        File::create(baseq3.path().join("pak4.pk3")).unwrap();
        File::create(baseq3.path().join("pak0.pk3")).unwrap();

        let resources = list_resources(baseq3.path()).unwrap();
        let names: Vec<&str> = resources
            .iter()
            .map(|r| r.file_name().unwrap().to_str().unwrap())
            .collect();

        assert_eq!(names, vec!["pak0.pk3", "pak1.pk3", "pak3.pk3", "pak4.pk3"]);
    }
}
