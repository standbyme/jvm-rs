use std::path::PathBuf;

struct DirEntry {
    path: PathBuf
}

impl Entry for DirEntry {
    fn new(path: String) -> DirEntry {
        DirEntry {
            path: Path::new(&path).to_owned()
        }
    }

    fn read_class(&self, class_file_name: String) -> Result<Vec<u8>, io::Error> {
        let filepath = Path::new(&self.path).join(class_file_name);
        let mut file = File::open(filepath)?;
        let meta = file.metadata()?;
        let mut buf = Vec::<u8>::with_capacity(meta.len() as usize);
        file.read_to_end(&mut buf);
        Ok(buf)
    }
}


#[derive(Debug)]
pub struct Classpath {
    user: DirEntry,
}

impl Classpath {
    pub fn read_class(&self, classname: String) -> Result<Vec<u8>, io::Error> {
        let class_file_name = classname + ".class";
        self.user.read_class()
    }
}

fn parse_user_classpath(cp_opt: Option<String>) -> impl Entry {
    new_entry(cp_opt.unwrap_or(".".to_owned()))
}

fn get_jre(jre_opt: Option<String>) -> String {
    match jre_opt {
        Some(ref jre) if exists(jre) => { jre.to_string() }
        _ => {
            if exists("./jre") { "./jre".to_string() } else {
                match env::var_os("JAVA_HOME") {
                    Some(java_home) => { Path::new(&java_home).join("jre").to_str().unwrap().to_string() }
                    None => panic!("Can not find JRE folder")
                }
            }
        }
    }
}

pub fn parse(jre_opt: Option<String>, cp_opt: Option<String>) -> Classpath {
    let jre = get_jre(jre_opt);
    let user = parse_user_classpath(cp_opt);
    Classpath {
        user,
    }
}