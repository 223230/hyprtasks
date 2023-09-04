use std::sync::Arc;
use std::sync::Mutex;

use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::Result;
use hyprland::event_listener::EventListener;

mod entries;
use entries::EntryList;

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
        e.rename_task(address, title, None);
        e.print();
    });

    let _ = listener.start_listener();
    
    Ok(())
}