use crate::app::state::FileEntry;

pub fn get_icon(file: &FileEntry) -> &'static str {
    if file.is_dir {
        return "󰉋 ";
    }

    match file.path.extension().and_then(|e| e.to_str()) {
        Some("rs")                           => " ",
        Some("toml")                         => " ",
        Some("md")                           => "󰍔 ",
        Some("json")                         => " ",
        Some("js")                           => " ",
        Some("ts")                           => " ",
        Some("py")                           => " ",
        Some("html")                         => " ",
        Some("css")                          => " ",
        Some("png") | Some("jpg")            => "󰋩 ",
        Some("mp4") | Some("mkv")            => "󰎁 ",
        Some("zip") | Some("tar")            => "󰗄 ",
        _                                    => "󰈔 ",
    }
}