use crate::ast;
use crate::file;
use downloader::Downloader;

pub struct Interpreter{
    name: String,
    version: String,
    mcversion: String,
    metadata: file::File,
    path: String,
    root: bool,
    lc: u16,
    dlc: u16,
    instc: u16,
    errorc: u16,
    folderc: u16,
    file_list: Vec<String>
}

impl Interpreter{
    /// Create new Interpreter
    pub fn new() ->Interpreter{
        Interpreter{
            metadata: file::File::new(String::new(), String::new()),
            name: String::new(),
            version: String::new(),
            mcversion: String::new(),
            path: String::new(),
            root: true,
            lc: 0,
            dlc: 0,
            instc: 0,
            errorc: 0,
            folderc: 0,
            file_list: Vec::new()
        }
    }

    /// Evaluate AST and perform operations
    /// # Example
    /// ```
    /// let ast = parser::parse(file_content).unwrap();
    /// let mut interpreter = operations::Interpreter::new();
    /// interpreter.eval(ast);
    /// ```
    pub fn eval(&mut self, ast: Vec<ast::Node>){
        for node in ast{
            self.lc += 1;
            match node{
                ast::Node::Statement(x) => {
                    self.instc += 1;
                    self.eval_statement(x);
                },
                ast::Node::Section(mut x) => {
                    self.folderc +=1;
                    self.path = if self.root==true{format!("{}/{}", self.name, x.path.clone())}else{x.path.clone()};
                    x.path = self.path.clone();
                    x.create();
                },
                ast::Node::Error => {println!("Error on {}th non-blank line", self.lc);self.errorc+=1;}
            }
        }
        self.write_meta();
        println!("----------------------------------------");
        println!("PROCESS REPORT");
        println!("----------------------------------------");
        println!("Total Non-Empty Line Count: {}", self.lc);
        println!("Total Statements: {}", self.instc);
        println!("Total Folders: {}", self.folderc);
        println!("Total Downloads: {}", self.dlc);
        println!("Total Errors: {}", self.errorc);
        println!("----------------------------------------");
    }

    /// Write metafile containing name, version, minecraft version and file list
    fn write_meta(&mut self){
        self.metadata.path = if self.root==true{
            format!("{}/.metadata", self.name)}else{format!(".metadata")
        };

        self.metadata.content = format!(
            "[Modpack_Info]
            Modpack Name: {}\n
            Modpack Version: {}\n
            Minecraft Version: {}",
            self.name, self.version, self.mcversion
        );

        if self.file_list.len()>0{
            self.metadata.content.push_str("\n[files]");
            for string in &self.file_list{
                self.metadata.content.push_str(format!("\n{}", string));
            }
        }

        self.metadata.write();
    }

    /// Evaluate Statement and perform operations
    fn eval_statement(&mut self, statement: ast::Statement){
        match statement.key.as_str(){
            "name" => self.set_name(statement.value, statement.attributes),
            "version" => self.version = statement.value,
            "mcversion" => self.mcversion = statement.value,
            "dl" => self.download(statement.value, statement.attributes),
            _ => println!("unknown statement: {}", statement.key)
        }
    }

    /// Set the name of modpack
    fn set_name(&mut self, value: String, attributes: Vec<ast::Attribute>){
        if attributes.len() > 0 {for attribute in attributes{
            match attribute.key.as_str(){
                "root" => {match attribute.value.as_str(){
                    "true"  => self.root = true,
                    "false" => self.root = false,
                    "1"     => self.root = true,
                    "0"     => self.root = false,
                    _ => println!("unknown boolean for root: {}", attribute.value)
                }},
                _ => println!("unknown attribute: {}", attribute.key)
            }
        }}
        self.name = value;
    }

    /// Download operation
    fn download(&mut self, value: String, attributes: Vec<ast::Attribute>){
        if attributes.len() > 0 {for attribute in attributes{
            match attribute.key.as_str(){
                "mode" => {match attribute.value.as_str(){
                    "normal" => (),
                    "http"   => (),
                    "HTTP"   => (),
                    "https"  => (),
                    "HTTPS"  => (),
                    "curseforge" => {self.download_from_curseforge(value); return;},
                    _ => println!("unknown mode: {}", attribute.value)
                }},
                _ => println!("unknown attribute: {}", attribute.key)
            }
        }}
        let mut downloader = Downloader::builder()
            .download_folder(std::path::Path::new(&self.path))
            .build()
            .unwrap();

        let dl1 = downloader::Download::new(&value);
        let result = downloader.download(&[dl1]).unwrap();

        for r in result {
            match r {
                Err(e) => {print!("Error occured! {}", e.to_string());self.errorc+=1;},
                Ok(s) => {print!("Success: {}", &s);self.dlc += 1;},
            };
        }
    }
    
    /// Get the link of actual file using ID and filename
    // dev's note: I need to clean this...
    fn download_from_curseforge(&mut self, value: String){
        let value = value.split("/");
        let value = value.collect::<Vec<&str>>();
        let len = value[0].len() - 3;
        let num = String::from(value[0]);
        let text = String::from(value[1]);
        let p1 = &num[..len];
        let p2 = &num[len..];
        let p2 = if &p2[0..1] == "0"{&p2[1..]} else {p2};
        let p2 = if &p2[0..1] == "0"{&p2[1..]} else {p2};
        let value = format!("https://media.forgecdn.net/files/{}/{}/{}", p1, p2, text);
        println!("{}", value);

        let mut downloader = Downloader::builder()
            .download_folder(std::path::Path::new(&self.path))
            .build()
            .unwrap();

        let dl1 = downloader::Download::new(&value);
        let result = downloader.download(&[dl1]).unwrap();

        for r in result {
            match r {
                Err(e) => {print!("Error occured! {}", e.to_string());self.errorc+=1;},
                Ok(s) => {print!("Success: {}", &s);self.dlc += 1;},
            };
        }
    }
}