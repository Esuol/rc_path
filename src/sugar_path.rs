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

    fn relative(&self, to: impl AsRef<Path>) -> PathBuf {
        let base = to.as_ref().absolutize();
        let target = self.absolutize();
        let mut ret = PathBuf::new();
        if base == target {
            return PathBuf::new();
        } else {
            let base_components = base
                .components()
                .into_iter()
                .filter(|com| {
                    matches!(
                        com,
                        Component::Normal(_) | Component::Prefix(_) | Component::RootDir
                    )
                })
                .collect::<Vec<_>>();
            let target_components = target
                .components()
                .into_iter()
                .filter(|com| {
                    matches!(
                        com,
                        Component::Normal(_) | Component::Prefix(_) | Component::RootDir
                    )
                })
                .collect::<Vec<_>>();

            let longest_len = if base_components.len() > target_components.len() {
                base_components.len()
            } else {
                target_components.len()
            };
            let mut i = 0;
            while i < longest_len {
                let from_component = base_components.get(i);
                let to_component = target_components.get(i);

                if cfg!(target_family = "windows") {
                    if let Some(Component::Normal(from_seg)) = from_component {
                        if let Some(Component::Normal(to_seg)) = to_component {
                            if from_seg.to_ascii_lowercase() == to_seg.to_ascii_lowercase() {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }

                if from_component != to_component {
                    break;
                }

                i += 1;
            }

            let mut from_start = i;
            while from_start < base_components.len() {
                ret.push("..");
                from_start += 1;
            }

            let mut to_start = i;
            while to_start < target_components.len() {
                ret.push(target_components[to_start]);
                to_start += 1;
            }

            ret
        }
    }
}
