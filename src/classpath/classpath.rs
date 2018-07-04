use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

trait Entry {
    fn new(path: String) -> Self;
    fn read_class(&self, class_file_name: String) -> Result<Vec<u8>, io::Error>;
}

#[derive(Debug)]
struct DirEntry {
    path: PathBuf,
}

impl Entry for DirEntry {
    fn new(path: String) -> DirEntry {
        DirEntry {
            path: Path::new(&path).to_owned(),
        }
    }

    fn read_class(&self, class_file_name: String) -> Result<Vec<u8>, io::Error> {
        let filepath = Path::new(&self.path).join(class_file_name);
        let mut file = File::open(filepath)?;
        let meta = file.metadata()?;
        let mut buf = Vec::<u8>::with_capacity(meta.len() as usize);
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[derive(Debug)]
pub struct ClassPath {
    user: DirEntry,
}

impl ClassPath {
    pub fn read_class(&self, classname: &str) -> Result<Vec<u8>, io::Error> {
        let class_file_name = classname.replace('.', "/") + ".class";
        self.user.read_class(class_file_name)
    }
}

fn parse_user_classpath(cp_opt: Option<String>) -> DirEntry {
    new_entry(cp_opt.unwrap_or(".".to_owned()))
}

fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn get_jre(jre_opt: Option<String>) -> String {
    match jre_opt {
        Some(ref jre) if exists(jre) => jre.to_string(),
        _ => {
            if exists("./jre") {
                "./jre".to_string()
            } else {
                match env::var_os("JAVA_HOME") {
                    Some(java_home) => Path::new(&java_home)
                        .join("jre")
                        .to_str()
                        .unwrap()
                        .to_string(),
                    None => panic!("Can not find JRE folder"),
                }
            }
        }
    }
}

fn new_entry(path: String) -> DirEntry {
    DirEntry::new(path)
}

pub fn parse(jre_opt: Option<String>, cp_opt: Option<String>) -> ClassPath {
    let user = parse_user_classpath(cp_opt);
    ClassPath { user }
}
