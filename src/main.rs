use std::sync::Arc;
use std::sync::Mutex;

use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::Result;
use hyprland::shared::Address;
use hyprland::event_listener::EventListener;

use serde::{Deserialize, Serialize};

struct EntryList {
    entries: Vec<Entry>
}

impl EntryList {
    fn new() -> EntryList {
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

    fn add_task(&mut self, address: Address, class: String, title: Option<String>) {
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

    fn remove_task(&mut self, address: Address) {
        self.entries.iter_mut().for_each(|entry| {
            entry.tasks.retain(|task| {
                task.address.clone().as_vec() != address.clone().as_vec()
            });
        });
        self.entries.retain(|e| e.tasks.len() > 0);
    }

    fn rename_task(&mut self, address: Address, title: String) {
        self.entries.iter_mut().for_each(|entry| {
            entry.tasks.iter_mut().filter(|task| {
                task.address.clone().as_vec() == address.clone().as_vec()
            }).for_each(|task| {
                task.title = title.clone();
            });
        });
    }

    /// Prints in json format
    fn print(&self) {
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

fn main() -> Result<()> {
    let entry_list = Arc::new(Mutex::new(EntryList::new()));
    entry_list.lock().unwrap().print();

    let mut listener = EventListener::new();
    
    let e = entry_list.clone();
    listener.add_window_open_handler(move |data| {
        let mut e = e.lock().unwrap();
        e.add_task(
            data.window_address,
            data.window_class,
            Some(data.window_title)
        );
        e.print();
    });
    
    let e = entry_list.clone();
    listener.add_window_close_handler(move |address| {
        let mut e = e.lock().unwrap();
        e.remove_task(address);
        e.print();
    });

    let e = entry_list.clone();
    listener.add_window_title_change_handler(move |address| {
        let title = Clients::get().unwrap().iter().filter(|c| {
            c.address.clone().as_vec() == address.clone().as_vec()
        }).nth(0).unwrap().title.clone();

        let mut e = e.lock().unwrap();
        e.rename_task(address, title);
        e.print();
    });

    let _ = listener.start_listener();
    
    Ok(())
}