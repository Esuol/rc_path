use std::path::Path;

pub trait AsPath {
    fn as_path(&self) -> &Path;
}

/// 为所有实现了AsRef<str> trait的类型T实现了AsPath trait。AsRef是一个标准库中的trait，它用于将一个类型转换为另一个类型的引用。在这个实现中，as_path方法通过调用Path::new函数将self（一个实现了AsRef<str>的类型）转换为一个Path引用。
impl<T: AsRef<str>> AsPath for T {
    fn as_path(&self) -> &Path {
        Path::new(self.as_ref())
    }
}
