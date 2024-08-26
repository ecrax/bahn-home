// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashSet, path::PathBuf};

use chrono::{Duration, NaiveDateTime, Utc};
use railway_core::{HyperRustlsRequester, HyperRustlsRequesterBuilder, JourneysOptions, LocationsOptions, Mode, Provider, Requester};
use railway_provider_hafas::{
    client::HafasClient, profile::db::DbProfile, profile::kvb::KvbProfile,
};
use rspc::Config;
use serde::Serialize;
use specta::Type;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

struct AppData {
    db_client: HafasClient<HyperRustlsRequester>,
    kvb_client: HafasClient<HyperRustlsRequester>,
}

#[derive(Debug, Type, Serialize)]
struct Journey {
    from: String,
    to: String,
    departure: NaiveDateTime,
    arrival: NaiveDateTime,
    planned_depature: NaiveDateTime,
    planned_arrival: NaiveDateTime,
    line: String,
}

impl Journey {
    async fn new<R: Requester>(client: &HafasClient<R>, start: &str, end: &str, modes: HashSet<Mode>) -> Self {
        let from = client
                    .locations(LocationsOptions {
                        query: start.to_string(),
                        results: 1,
                        language: Some("de".to_string()),
                    })
                    .await
                    .unwrap()
                    .pop()
                    .unwrap();

                let to = client
                    .locations(LocationsOptions {
                        query: end.to_string(),
                        results: 1,
                        language: Some("de".to_string()),
                    })
                    .await
                    .unwrap()
                    .pop()
                    .unwrap();

                let res = client
                    .journeys(
                        from,
                        to,
                        JourneysOptions {
                            departure: Some(
                                Utc::now()
                                    .with_timezone(&chrono_tz::Europe::Berlin)
                                    .checked_add_signed(Duration::minutes(15))
                                    .unwrap(),
                            ),
                            products: modes.into(),
                            ..Default::default()
                        },
                    )
                    .await
                    .unwrap();

                Journey {
                    from: start.to_string(),
                    to: end.to_string(),
                    departure: res
                        .journeys
                        .first()
                        .unwrap()
                        .legs
                        .first()
                        .unwrap()
                        .departure
                        .unwrap()
                        .clone()
                        .naive_local(),
                    arrival: res
                        .journeys
                        .first()
                        .unwrap()
                        .legs
                        .last()
                        .unwrap()
                        .arrival
                        .unwrap()
                        .clone()
                        .naive_local(),
                    planned_arrival: res
                        .journeys
                        .first()
                        .unwrap()
                        .legs
                        .last()
                        .unwrap()
                        .planned_arrival
                        .unwrap()
                        .clone()
                        .naive_local(),
                    planned_depature: res
                        .journeys
                        .first()
                        .unwrap()
                        .legs
                        .first()
                        .unwrap()
                        .planned_departure
                        .unwrap()
                        .clone()
                        .naive_local(),
                    line: res
                        .journeys
                        .first()
                        .unwrap()
                        .legs
                        .first()
                        .unwrap()
                        .line
                        .as_ref()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
    }
}

#[tokio::main]
async fn main() {
    // system tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(system_tray_menu);

    // rspc
    let router = <rspc::Router<AppData>>::new()
        .config(Config::new().export_ts_bindings(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/bindings.ts"),
        ))
        .query("version", |t| t(|_, _: ()| env!("CARGO_PKG_VERSION")))
        .query("greet", |t| t(|_, name: String| format!("Hello, {name}!")))
        .query("db", |t| {
            t(|ctx, _: ()| async move {
                Journey::new(&ctx.db_client, "Köln Messe/Deutz", "Köln-Weiden West", HashSet::from([Mode::SuburbanTrain])).await
            })
        })
        .query("kvb", |t| {
            t(|ctx, _: ()| async move {
                Journey::new(&ctx.kvb_client, "Bahnhof Deutz/Messe LANXESS arena, Köln", "Weiden West, Köln", HashSet::from([Mode::RegionalTrain])).await
            })
        })
        .build()
        .arced();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // blur window
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(12.))
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .system_tray(tray)
        .plugin(rspc_tauri::plugin(router, |_| AppData {
            db_client: HafasClient::new(DbProfile {}, HyperRustlsRequesterBuilder::default()),
            kvb_client: HafasClient::new(KvbProfile {}, HyperRustlsRequesterBuilder::default()),
        }))
        .plugin(tauri_plugin_positioner::init())
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
