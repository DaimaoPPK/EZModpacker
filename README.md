Please check out [pest](https://github.com/pest-parser/pest). It is such a cool and simple to use library for parsing.

# Overview
Install modpack from .modpackinfo files which can be easily share in web.

# Installation
1. ```git clone https://github.com/DaimaoPPK/EZModpacker.git```
2. Install Rust and Cargo from [this](https://www.rust-lang.org/tools/install).
3. ```cd EZModpacker``` and ```cargo build --release```
4. Output file is at `output/release/ezmodpacker`

# Syntax
**NOTE: REPLACE ${x} WITH RESPECTIVE VALUE**

## Statement
```name: ${x}```  
Name of modpack

```version: ${x}```  
Version of modpack

```mcversion: ${x}```  
Version of minecraft

```dl: ${x}```  
Download from link(or other values)

## Section
```[${x}]```  
downloads following this will be put in ${x} directory

## Attribute
`<>` following left value is attribute field. You can seperate each attribute with comma.

```name<root=false>: ${x}```  
Doesn't generate root folder

```download<mode=${x}>: ${https_link}```  
`normal` = Download from http or https.

`curseforge` = Download from curseforge and value will be ${number/filename} where number is 7 or 6 digits ID. You can see them at the end of curseforge mod file link. Filename is name of file.   
E.G:  3222876/journeymap-1.16.5-5.7.1.jar
