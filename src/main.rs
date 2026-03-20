use std::env;
use std::fs;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Usage: Tidy <FolderName>");
        return;
    }
    let folder = &args[1];

    let entries = fs::read_dir(folder).expect("Could not read directory");
    for entry in entries {
        let entry = entry.expect("Could not read entry");
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.starts_with(".") {
            continue;
        }

        let extension = path.extension();

        let folder_name = match extension {
            Some(ext) => match ext.to_str().unwrap() {
                "jpg" | "jpeg" | "png" | "gif" => "Pictures",
                "mp4" | "mkv" | "avi" | "mov" => "Videos",
                "mp3" | "flac" | "wav" | "ogg" => "Music",
                "pdf" | "docx" | "txt" | "doc" => "Documents",
                "zip" | "tar" | "gz" | "rar" => "Archives",
                "py" | "lua" | "html" | "css" | "c" | "cpp" | "js" | "h" | "hpp" | "java"
                | "class" | "jar" | "sh" => "Code",
                _ => "Other",
            },
            None => "Other",
        };
        let home = std::env::var("HOME").expect("Could not find HOME directory");
        let dest_dir = Path::new(&home).join(folder_name);
        fs::create_dir_all(&dest_dir).expect("Could not create directory");

        let file_name = path.file_name().unwrap();
        let dest_path = dest_dir.join(file_name);

        fs::rename(&path, &dest_path).expect("Could not move file");

        println!("Moved {:?} into {}/", file_name, folder_name);
    }
}

