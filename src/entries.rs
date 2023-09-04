use hyprland::shared::Address;
use serde::{Deserialize, Serialize};
use hyprland::data::*;
use hyprland::prelude::*;

pub struct EntryList {
    entries: Vec<Entry>
}

impl EntryList {
    pub fn new() -> EntryList {
        let mut entry_list = EntryList { entries: vec![] };

        match Clients::get() {
            Ok(clients) => clients.to_vec().iter()
                .filter(|client| { client.mapped })
                .for_each(|client| {
                    entry_list.add_task(
                        client.address.to_owned(),
                        client.class.to_owned(),
                        Some(client.title.to_owned())
                    );
                }),
            Err(e) => eprintln!("{}",e)
        };

        return entry_list;
    }

    pub fn add_task(&mut self, address: Address, class: String, title: Option<String>) {
        let title = title.unwrap_or("".to_string());
        let task = Task {
            title: title.clone(),
            address: address
        };

        match self.entries
            .iter_mut()
            .find(|entry| entry.class == class) {
            Some(entry) => {
                entry.tasks.push(task)
            }
            None => {
                self.entries.push(Entry {
                    title: title,
                    class: class,
                    tasks: vec![task]
                })
            }
        };
    }

    pub fn remove_task(&mut self, address: Address) {
        self.entries.iter_mut().for_each(|entry| {
            entry.tasks.retain(|task| {
                task.address.clone().as_vec() != address.clone().as_vec()
            });
        });
        self.entries.retain(|e| e.tasks.len() > 0);
    }

    pub fn rename_task(&mut self, address: Address, title: String, initial_title: Option<String>) {
        self.entries.iter_mut().for_each(|entry| {
            match initial_title.clone() {
                Some(t) => entry.title = t,
                None => {}
            };

            entry.tasks.iter_mut().filter(|task| {
                task.address.clone().as_vec() == address.clone().as_vec()
            }).for_each(|task| {
                task.title = title.clone();
            });
        });
    }

    /// Prints in json format
    pub fn print(&self) {
        match serde_json::to_string(&self.entries) {
            Ok(json) => println!("{}", json),
            Err(_) => println!("[]")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    title: String,
    class: String,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    address: Address
}