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

    fn absolutize(&self) -> Cow<Path> {
        if self.is_absolute() {
            self.normalize()
        } else if cfg!(target_family = "windows") {
            let mut components = self.components();
            if matches!(components.next(), Some(Component::Prefix(_)))
                && matches!(components.next(), Some(Component::RootDir))
            {
                let mut components = self.components().into_iter().collect::<Vec<_>>();
                components.insert(1, Component::RootDir);
                component_vec_to_path_buf(components)
                    .into_normalize()
                    .into()
            } else {
                let mut cwd = CWD.clone();
                cwd.push(self);
                cwd.into_normalize().into()
            }
        } else {
            let mut cwd = CWD.clone();
            cwd.push(self);
            cwd.into_normalize().into()
        }
    }
}
