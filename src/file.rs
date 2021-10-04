use std::fs;

#[derive(Clone, Debug)]
pub struct File{
    pub path: String,
    pub content: String
}

#[derive(Clone, Debug)]
pub struct Folder{
    pub path: String
}

impl File{
    pub fn new(path: String, content: String) -> File {File{path,content}}
    
    pub fn read(path: String) -> File{
        File::new(
            path.clone(),
            fs::read_to_string(path)
                .expect("Something went wrong while reading File")
        )
    }

    pub fn write(&self){
        fs::File::create(&self.path)
            .expect("Something went wrong while creating File");
        fs::write(&self.path, &self.content)
            .expect("Something went wrong while writing File");
    }
}

impl Folder{
    pub fn new(path: String) -> Folder {Folder{path}}

    pub fn create(&self){
        fs::create_dir_all(self.path.clone())
            .expect("Something went wrong while creating folder");
    }
}