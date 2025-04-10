
use gloo::storage::{LocalStorage, Result, Storage};

const THEME_KEY: &str = "app-theme";

fn get_theme_from_storage() -> Result<String> {
    LocalStorage::get(THEME_KEY)
}

fn set_theme_in_storage(theme: &str) -> Result<()> {
    LocalStorage::set(THEME_KEY, theme)
}