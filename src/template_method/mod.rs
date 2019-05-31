//!# 模版方法模式
//!
//! 模版方法模式使用继承机制，把通用步骤和通用方法放到父类中，把具体实现延迟到子类中实现。使得实现符合开闭原则。
//!
//! Rust中可以简单地在trait中添加默认方法即可

pub trait Downloader {
    fn custom_download(&self, uri: &str);
    /// Template method
    fn download(&self, uri: &str) {
        println!("prepare to download");
        self.custom_download(uri);
        self.save();
        println!("download finished");
    }
    fn save(&self) {
        println!("default save")
    }
}

pub struct HTTPDownloader {}

impl Downloader for HTTPDownloader {
    fn custom_download(&self, uri: &str) {
        println!("download {} via http", uri);
    }
}

pub struct FTPDownloader {}

impl Downloader for FTPDownloader {
    fn custom_download(&self, uri: &str) {
        println!("download {} via ftp", uri);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let downloader = HTTPDownloader {};
        downloader.download("http://example.com/abc.zip");
        // Output:
        // prepare to download
        // download http://example.com/abc.zip via http
        // default save
        // download finished

        let downloader = FTPDownloader {};
        downloader.download("ftp://example.com/abc.zip");

        // Output:
        // prepare to download
        // download ftp://example.com/abc.zip via ftp
        // default save
        // download finished
    }
}