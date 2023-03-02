pub mod error;

use std::{collections::HashMap, path::{Path, PathBuf}, fs::{File, OpenOptions, create_dir}, io::{BufWriter, SeekFrom, BufReader, Seek, BufRead, Write, self}, result};
pub use error::Result;
use serde::{Deserialize, Serialize};

use crate::error::KvsError;

#[derive(Serialize,PartialEq, Deserialize, Debug, Clone)]
pub enum Command {
    GET,
    SET,
    RM
}

#[derive(Serialize,PartialEq, Deserialize, Debug, Clone)]
pub struct Entry {
    command: Command,
    key: String, 
    value: String
}

pub struct KvStore {
    file_path: PathBuf,
    pub mem_map: HashMap<String, usize>
}


impl KvStore {
    pub fn open(dir_path: &Path) -> Result<Self> {
        let mut file_path = dir_path.to_path_buf();
        create_dir(&file_path);
        file_path.push("db.txt");

        
        if !file_path.as_path().exists() {
            File::create(&file_path)?;
        }

        let kvs = KvStore {
            file_path,
            mem_map: HashMap::new()
        };

        Ok(kvs)
    } 

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(pos) = self.mem_map.get(&key) {
            let value = self.get_value_by_pos(*pos);
            Ok(value)
        }
        else {
            match self.seek_value_by_key(key.clone()) {
                Some((pos, value)) => {
                    self.mem_map.insert(key.clone(), pos);
                    Ok(Some(value))
                },
                None => Ok(None),
            }
        }
    }


    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.mem_map.remove(&key);
        self.write_entry(Command::SET, key, value).unwrap();
        Ok(())

    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let Some(pos) = self.seek_value_by_key(key.clone()) else {
            return Err(KvsError::KeyNotFound);
        };
        self.write_entry(Command::RM, key, String::from("removed")).unwrap();
        Ok(())

    }

    fn write_entry(&mut self, command: Command, key: String, value: String) -> Result<()>{
        let entry = Entry {
            command,
            key,
            value
        };

        let entry_str = serde_json::to_string(&entry).unwrap();

        let mut writer = self.get_writer();

        writeln!(writer, "{}", entry_str);
        Ok(())

    }

    fn get_value_by_pos(&self, pos: usize) -> Option<String> {
        let mut file_reader = self.get_reader();
    
        match file_reader.nth(pos) {
            Some((_,entry)) => return Some(entry.unwrap().value),
            None => return None,
        }
    }
    
    fn seek_value_by_key(&self, key: String) -> Option<(usize, String)> {
        let file_reader = self.get_reader();

        let mut result:Option<(usize, String)> = None;
    
        for (pos,entry) in file_reader {
            let entry = entry.unwrap();
            match entry.command {
                Command::GET => {},
                Command::SET => {
                    if entry.key.eq(&key) {
                        result = Some((pos, entry.value));
                    }
                },
                Command::RM => {
                    if entry.key.eq(&key) {
                        result = None;
                    }
                },
            }
        }
        result
    }

    fn get_reader(&self) -> std::iter::Enumerate<serde_json::StreamDeserializer<'static, serde_json::de::IoRead<BufReader<File>>, Entry>> {
        let file = File::open(self.file_path.as_path()).unwrap();
        let reader = BufReader::new(file);
        let mut iterator = serde_json::Deserializer::from_reader(reader).into_iter::<Entry>().enumerate();
        iterator
    }

    fn get_writer(&self) -> BufWriter<File> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.file_path.as_path())
            .unwrap();
            
        BufWriter::new(file)
    }

    
}