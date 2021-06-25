#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{
  CustomMenuItem, Manager, Menu, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry,
};

use std::process::exit;
trait Params:
  tauri::Params<Event = String, Label = String, MenuId = MenuId, SystemTrayMenuId = TrayMenuId>
{
}
impl<P> Params for P where
  P: tauri::Params<Event = String, Label = String, MenuId = MenuId, SystemTrayMenuId = TrayMenuId>
{
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
enum TrayMenuId {
  Exit,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
enum MenuId {
  SaveLog,
  Exit,
}

fn main() {
  tauri::Builder::<String, String, MenuId, TrayMenuId, _, Wry>::new()
    .menu(
      Menu::new().add_submenu(Submenu::new(
        "File",
        Menu::new()
          .add_item(CustomMenuItem::new(MenuId::SaveLog, "Save Log"))
          .add_item(CustomMenuItem::new(MenuId::Exit, "Exit")),
      )),
    )
    .on_menu_event(|event| match event.menu_item_id() {
      MenuId::SaveLog => {}
      MenuId::Exit => exit(0),
    })
    .system_tray(SystemTray::new().with_menu(
      SystemTrayMenu::new().add_item(CustomMenuItem::new(TrayMenuId::Exit, "Exit OpenRefine")),
    ))
    .on_system_tray_event(move |app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        app.get_window("main").unwrap().show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => match id {
        TrayMenuId::Exit => {
          exit(0);
        }
      },
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
