use std::{
    borrow::Cow,
    path::{Component, Path, PathBuf},
};

use crate::{
    utils::{component_vec_to_path_buf, normalize_to_component_vec, CWD},
    SugarPathBuf,
};

pub trait SugarPath {
    fn normalize(&self) -> Cow<Path>;

    fn absolutize(&self) -> Cow<Path>;

    fn relative(&self, to: impl AsRef<Path>) -> PathBuf;
}

impl SugarPath for Path {
    fn normalize(&self) -> Cow<Path> {
        let mut components = normalize_to_component_vec(self);
        if components.is_empty() {
            return Cow::Borrowed(Path::new("."));
        }

        if cfg!(target_family = "windows") {
            if components.len() == 1 && matches!(components[0], Component::Prefix(_)) {
                components.push(Component::CurDir);
            }
        }

        component_vec_to_path_buf(components).into()
    }
}
