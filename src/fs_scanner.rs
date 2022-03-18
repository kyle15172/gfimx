use walkdir::WalkDir;

pub struct FilesystemScanner;

impl FilesystemScanner {
    pub fn scan_dir(&self, dir: &str) {
        for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok()) 
        {
            let f_name = entry.file_name().to_string_lossy();
            println!("{}", f_name)
        }
    }
}