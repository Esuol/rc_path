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
}
