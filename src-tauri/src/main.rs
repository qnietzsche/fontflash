#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
/*#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/
use std::{env, path};
use std::borrow::Cow;
use tauri::{Manager};
use window_shadows::set_shadow;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use tauri::api::dir::is_dir;
use fonttools::font::{self, Font, Table};
use fonttools::name::{name, NameRecord, NameRecordID};
use serde::{Serialize, Deserialize};
use ttf_parser::Weight;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileData {
    has_patharg: bool,
    err: String,
    filepath: String,
    dir_files: Vec<String>,
    font_name: Vec<String>,
    font_weight: String
}


#[tauri::command]
fn get_args() -> FileData {
    let mut args: Vec<String> = env::args().collect();
    let _args = args;
    if _args.len()<2{
        return FileData{
            has_patharg: false,
            err: "".to_string(),
            filepath: "undefined".to_string(),
            dir_files: vec![],
            font_name: vec![],
            font_weight: "".to_string(),
        }
    }

    return get_data(_args[1].clone())
}

/*#[tauri::command]
fn get_data_from_path() -> FileData {

}*/
#[tauri::command]
fn get_data(path:String) -> FileData {
/*    let _path = path;

*/  println!("gettingdata");
     println!("{}", path.to_string());

    if check_extension(path.clone()){
        let data = std::fs::read(path.clone()).unwrap();
        let face = match ttf_parser::Face::parse(&data, 0) {
            Ok(f) => f,
            Err(e) => {
                eprint!("Error: {}.", e);
                std::process::exit(1);
            }
        };
        let mut family_names = Vec::new();
        for name in face.names() {
            if name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode() {
                if let Some(family_name) = name.to_string() {
                    let language = name.language();
                    family_names.push(format!(
                        "{} ({}, {})",
                        family_name,
                        language.primary_language(),
                        language.region()
                    ));
                }
            }
        }
        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.to_string());

        println!("Family names: {:?}", family_names);
        println!("Weight: {:?}", face.weight());
        println!("PostScript name: {:?}", post_script_name);

        let dir_list= get_dir(path.clone());
        println!("{:?}", dir_list);
        return FileData {
            has_patharg: true,
            err:"".to_string(),
            filepath: path.clone(),
            dir_files: dir_list,
            font_name: family_names,
            font_weight: convert_weight(face.weight())
        };

    }

    return FileData{
        has_patharg: false,
        err: "".to_string(),
        filepath: "undefined".to_string(),
        dir_files: vec![],
        font_name: vec![],
        font_weight: "".to_string(),
    }

/*    let data = std::fs::read(r"C:\Users\ym174\Desktop\A-OTF-AntiqueStd-AN3.otf").unwrap();
*/


/*    println!("The font name is {}", font_name);
*/

}

fn get_dir(path:String)->Vec<String>{
    let mut dir_list:Vec<String> = vec![];
    let filename = path.split(r"\").collect::<Vec<&str>>()[path.split(r"\").collect::<Vec<&str>>().len() - 1];
    let dirname = path.replace(filename, "");

    println!("{}", dirname.to_string());
    let target = path::PathBuf::from(dirname);

    for dir_entry in fs::read_dir(target).unwrap() {
        // dir_entry(Result<DirEntry, Error>型)をfile_path(PathBuf型)に変換する
        let entry = dir_entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file_path: String = entry.file_name().into_string().unwrap();
            /*            let file_path : &str = file_path.unwrap();
            */            println!("{:?}", file_path);
            let splited_name = file_path.split(r".").collect::<Vec<&str>>();
            if splited_name.len() > 1 {
                println!("{}", splited_name.len());
                let extension = splited_name[splited_name.len() - 1];
                if extension == "woff2" ||
                    extension == "woff" ||
                    extension == "ttf" ||
                    extension == "otf" {
                    dir_list.push(file_path.to_string());
                }
            }
        }
    }
    return dir_list
}

fn check_extension(path:String)->bool{
    let filename = path.split(r"\").collect::<Vec<&str>>()[path.split(r"\").collect::<Vec<&str>>().len() - 1];
    let splited_name = filename.split(r".").collect::<Vec<&str>>();
    if splited_name.len() > 1 {
        println!("{}", splited_name.len());
        let extension = splited_name[splited_name.len() - 1];
        if /*extension == "woff2" ||
            extension == "woff" ||*/
            extension == "ttf" ||
            extension == "otf" {
            return true
        }
    }
    return false
}

fn convert_weight(weight: Weight) -> String{
    return match weight {
        Weight::Thin => "100".to_string(),
        Weight::ExtraLight => "200".to_string(),
        Weight::Light => "300".to_string(),
        Weight::Normal => "400".to_string(),
        Weight::Medium => "500".to_string(),
        Weight::SemiBold => "600".to_string(),
        Weight::Bold => "700".to_string(),
        Weight::ExtraBold => "800".to_string(),
        Weight::Black => "900".to_string(),
        _ => "err".to_string()
    }
}

/*
fn get_metadata() -> FileMetaData{

}

*/
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");

            Ok(())
        })
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            let f = get_data(argv[1].clone());
            app.emit_all("instance_detection", f).unwrap();
            let window = app.get_window("main").unwrap();

            // 最小化されてる場合は解除。
            window.unminimize().expect("Failed to un-minimize!");
            // フォーカスを有効にする。
            window.set_focus().expect("Failed to set-on-top!");
        }))
        .invoke_handler(tauri::generate_handler![get_args,get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


/*fn get_metadata ->
*/