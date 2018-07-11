pub struct Command<'a> {
    pub class_name: String,
    pub cp_opt: Option<String>,
    pub jre_opt: Option<String>,
    pub args: Vec<&'a str>,
}
