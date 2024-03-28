use std::path::{Component, PathBuf};

use crate::utils::{component_vec_to_path_buf, normalize_to_component_vec, CWD};

pub trait SugarPathBuf {
    fn into_normalize(self) -> PathBuf;

    fn into_absolutize(self) -> PathBuf;
}

impl SugarPathBuf for PathBuf {
    fn into_normalize(self) -> PathBuf {
        let mut components = normalize_to_component_vec(&self);

        if (components.is_empty()) {
            return PathBuf::from(".");
        }

        // 这是通过cfg!(target_family = "windows")宏来实现的，如果当前的目标平台是Windows，它将返回true。
        // 代码将进一步检查向量components的长度是否为1，以及components的第一个元素是否匹配Component::Prefix(_)模式。Component::Prefix(_)是一个模式，它匹配Component::Prefix类型的值。_表示忽略Component::Prefix的内部值。
        if cfg!(target_family = "windows") {
            if components.len() == 1 && matches!(components[0], Component::Prefix(_)) {
                // components向量中添加一个Component::CurDir元素。Component::CurDir表示当前目录，也就是路径中的"."。
                components.push(Component::CurDir);
            }
        }

        component_vec_to_path_buf(components)
    }

    fn into_absolutize(self) -> PathBuf {
        if self.is_absolute() {
            self.into_normalize()
        } else if cfg!(target_family = "windows") {
            let mut components = self.components();
        }
    }
}
