use std::path::{Component, Path, PathBuf};

use once_cell::sync::Lazy;

pub(crate) static CWD: Lazy<PathBuf> = Lazy::new(|| std::env::current_dir().unwrap());

#[inline]
pub fn normalize_to_component_vec(path: &Path) -> Vec<Component> {
    // 创建名为components的可变变量，它是一个可以预览下一个元素的迭代器。
    let mut components = path.components().peekable();
    // components.size_hint().0调用返回一个元组的第一个元素，这个元组是迭代器components的大小提示。大小提示是一个包含下界和上界的元组，其中下界是迭代器剩余元素的最小数量，上界是剩余元素的最大数量（如果已知）。在这个例子中，.0获取了大小提示的下界。
    // 创建了一个向量ret，它的初始容量被设置为迭代器components剩余元素的最小数量。这意味着如果components的所有元素都被添加到ret中，ret可能不需要重新分配内存。
    let mut ret = Vec::with_capacity(components.size_hint().0);
    if let Some(c @ Component::Prefix(..)) = components.peek() {
        // 如果components的下一个元素是Component::Prefix，则将其添加到ret中。
        ret.push(*c);
        // 跳过components的下一个元素。
        components.next();
    }

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component);
            }
            Component::CurDir => {
                // 如果component是Component::CurDir，则跳过它。
            }
            c @ Component::ParentDir => {
                let is_last_none_or_prefix =
                    matches!(ret.last(), None | Some(Component::Prefix(_)));
                if is_last_none_or_prefix {
                    // 如果ret的最后一个元素是None或Component::Prefix，则将component添加到ret中。
                    ret.push(c);
                } else {
                    let is_last_root_dir = matches!(ret.last(), Some(Component::RootDir));
                    if !is_last_root_dir {
                        let is_last_parent_dir = matches!(ret.last(), Some(Component::ParentDir));
                        if is_last_parent_dir {
                            // 如果ret的最后一个元素是Component::ParentDir，则将component添加到ret中。
                            ret.push(c);
                        } else {
                            // 如果ret的最后一个元素是Component::CurDir，则将ret的最后一个元素弹出。
                            ret.pop();
                        }
                    }
                }
            }

            c @ Component::Normal(_) => {
                ret.push(c);
            }
        }
    }
    ret
}

#[inline]
pub fn component_vec_to_path_buf(components: Vec<Component>) -> PathBuf {
    components.into_iter().collect()
}
