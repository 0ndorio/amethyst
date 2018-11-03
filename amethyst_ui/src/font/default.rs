use std::fs;

use font_kit::handle::Handle as FontKitHandle;

use amethyst_assets::{AssetStorage, Loader, SimpleFormat};

use {
    font::systemfont::default_system_font,
    format::{FontAsset, FontHandle, TtfFormat},
};

/// Get the system default fonts.
/// If unable to, gets the local square.ttf font.
pub fn get_default_font(loader: &Loader, storage: &AssetStorage<FontAsset>) -> FontHandle {
    let system_font = default_system_font();

    match system_font {
        Ok(handle) => match handle {
            FontKitHandle::Path { path, .. } => {
                if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
                    let format = if file_name.ends_with(".ttf") || file_name.ends_with(".otf") {
                        Some(TtfFormat)
                    } else {
                        error!("System font '{}' has unknown format", file_name);
                        None
                    };

                    if let Some(format) = format {
                        match fs::read(&path) {
                            Ok(bytes) => match format.import(bytes, ()) {
                                Ok(data) => return loader.load_from_data(data, (), storage),
                                Err(err) => warn!("System font at '{}' cannot be loaded. Fallback to default. Error: {}", path.display(), err),
                            },
                        Err(err) => warn!("System font at '{}' is not available for use. Fallback to default. Error: {}", path.display(), err)
                    }
                    }
                } else {
                    warn!("Unable to get system font name");
                }
            }
            FontKitHandle::Memory { bytes, .. } => {
                let font_data = TtfFormat.import(bytes.to_vec(), ());
                match font_data {
                    Ok(data) => return loader.load_from_data(data, (), storage),
                    Err(e) => warn!("Failed to load default system font from bytes. Falling back to built-in.\nError: {:?}", e),
                }
            }
        },
        Err(e) => warn!(
            "Failed to find suitable default system font. Falling back to built-in.\nError: {:?}",
            e
        ),
    }

    return loader.load_from_data(
        TtfFormat
            .import(include_bytes!("./square.ttf").to_vec(), ())
            .unwrap(),
        (),
        storage,
    );
}
