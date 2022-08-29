use crate::traits::FileIO;
use std::{collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    current_vault: Option<String>,
    vaults: HashMap<String, PathBuf>
}

impl Default for Data {
    fn default() -> Self {
        Data {
            current_vault: None, 
            vaults: HashMap::new()
        }
    }
}

impl FileIO for Data {
    fn path(&self) -> PathBuf {
        if let Some(dirs) = ProjectDirs::from("com", "", "jot") {
            let mut path = dirs.data_dir().to_path_buf();
            path.push("data");
            path
        } else {
            panic!("current path couldn't be generated")
        }
    }
}

impl Data {
    pub fn get_current_vault(&self) -> Option<&String> {
        self.current_vault.as_ref()
    }

    pub fn set_current_vault(&mut self, vault: Option<String>) {
        self.current_vault = vault;
        self.store()
    }

    pub fn get_vaults(&self) -> &HashMap<String, PathBuf> {
        &self.vaults
    }

    pub fn get_vault_location(&self, name: &str) -> Option<&PathBuf> {
        self.vaults.get(name)
    }

    pub fn vault_exists(&self, name: &str) -> bool {
        self.vaults.contains_key(name)
    }

    pub fn add_vault(&mut self, name: String, location: PathBuf) {
        self.vaults.insert(name, location);
        self.store()
    }

    pub fn delete_vault(&mut self, name: &str) {
        self.vaults.remove(name);
        self.store()
    }

    pub fn rename_vault(&mut self, name: &str, new_name: String) {
        let value = self.vaults.remove(name);
        self.vaults.insert(new_name, value.unwrap());
        self.store()
    }

    pub fn set_vault_location(&mut self, name: &String, new_location: PathBuf) {
        self.vaults.insert(name.to_owned(), new_location);
        self.store()
    }   
}