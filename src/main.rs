use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::Result;
use hyprland::shared::Address;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EntryList {
    entries: Vec<Entry>
}

impl EntryList {
    fn new() -> EntryList {
        let mut entries: Vec<Entry> = Vec::new();

        match Clients::get() {
            Ok(clients) => clients.to_vec().iter()
                // Skip clients which shouldn't be listed (Dialogs, etc.)
                .filter(|client| { client.mapped })
                .for_each(|client| {
                    // Create a task for the client
                    let task = Task{
                        title: client.title.to_owned(),
                        address: client.address.to_owned()
                    };
    
                    // Create a task for the client
                    let task_entry = entries
                        .iter_mut()
                        .find(|entry| entry.class == client.class);
    
                    match task_entry {
                        Some(entry) => {
                            entry.tasks.push(task)
                        }
                        None => {
                            entries.push(Entry {
                                title: client.initial_title.to_owned(),
                                class: client.class.to_owned(),
                                tasks: vec![task]
                            })
                        }
                    };
                }),
            Err(e) => eprintln!("{}",e)
        };

        return EntryList { entries }
    }

    fn print(&self) {
        // Serialize entries to a JSON string.
        match serde_json::to_string(&self.entries) {
            Ok(json) => println!("{}", json),
            Err(_) => println!("[]")
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Entry {
    title: String,
    class: String,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
struct Task {
    title: String,
    address: Address
}

fn main() -> Result<()> {
    let clients = Clients::get()?.to_vec();
    let clients_it = clients.iter();

    let e = EntryList::new();
    e.print();
    
    Ok(())
}