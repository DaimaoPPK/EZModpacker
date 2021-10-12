/*
* ISC License
*
* Copyright <2021> <Phone Pyae Kyaw>
*
* Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

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