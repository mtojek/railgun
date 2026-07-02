use regex::Regex;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
    u32,
};
struct ResourceManager {
    pk3s: Vec<Archive>,
    resources: HashMap<String, ArchiveRef>,
}

struct Archive {
    path: PathBuf,
}

struct ArchiveRef {
    archive_id: usize,
    index: usize,
}

pub fn load_resources(baseq_dir: &Path) -> io::Result<ResourceManager> {
    let pk3s = list_archives(baseq_dir)?;
    let resources = build_resources(pk3s);

    pk3s.iter().for_each(|pk3| {
        println!("{:?}", pk3.path);
    });

    Ok(ResourceManager {
        pk3s: pk3s,
        resources: resources,
    })
}

fn build_resources(pk3s: Vec<Archive>) -> HashMap<String, ArchiveRef> {
    let resources = HashMap::new();

    for (i, pk3) in pk3s.iter().enumerate() {}

    resources
}

fn list_archives(baseq_dir: &Path) -> io::Result<Vec<Archive>> {
    println!("baseq3 dir: {}", baseq_dir.display());

    let entries = baseq_dir.read_dir()?;

    let mut pk3s = Vec::new();
    let pk3_regex = Regex::new(r"^pak\d+\.pk3$").unwrap();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            if pk3_regex.is_match(filename) {
                let archive = Archive { path: path };
                pk3s.push(archive);
            }
        }
    }

    pk3s.sort_by_key(|pk3| {
        pk3.path
            .file_stem() // no extension: pak<N>
            .and_then(|f| f.to_str())
            .and_then(|s| s.strip_prefix("pak"))
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(u32::MAX)
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

        let resources = list_archives(baseq3.path()).unwrap();
        let names: Vec<&str> = resources
            .iter()
            .map(|r| r.path.file_name().unwrap().to_str().unwrap())
            .collect();

        assert_eq!(names, vec!["pak0.pk3", "pak1.pk3", "pak3.pk3", "pak4.pk3"]);
    }

    #[test]
    fn empty_directory_returns_empty_list() {
        let baseq3 = tempdir().unwrap();

        let resources = list_archives(baseq3.path()).unwrap();
        let names: Vec<&str> = resources
            .iter()
            .map(|r| r.path.file_name().unwrap().to_str().unwrap())
            .collect();

        assert!(names.is_empty())
    }

    #[test]
    fn non_existing_directory_returns_error() {
        let non_existing = Path::new("foobar");
        let result = list_archives(non_existing);
        assert!(result.is_err())
    }
}
