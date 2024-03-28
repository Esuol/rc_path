use std::path::{Component, PathBuf};

use crate::utils::{component_vec_to_path_buf, normalize_to_component_vec, CWD};

pub trait PathBuf {
    fn into_normalize(self) -> PathBuf;

    fn into_absolutize(self) -> PathBuf;
}
