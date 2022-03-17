pub struct fileattr {
    pub name: String,
    pub path: String,
    pub attr: String,
}

impl fileattr {
    pub fn init(name: String, path: String, attr: String) -> Self {
        fileattr {
            name: name,
            path: path,
            attr: attr,
        }
    }
}
