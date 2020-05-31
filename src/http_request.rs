use {
    crate::{Request, Result},
    std::net::TcpStream,
};

pub trait HTTPRequest: Send {
    fn method(&self) -> &str;
    fn path(&self) -> &str;
    fn full_path(&self) -> &str;
    fn body(&self) -> &str;
    fn set_arg(&mut self, key: String, value: String);
    fn arg(&self, name: &str) -> Option<&str>;
    fn header(&self, name: &str) -> Option<&str>;
    fn form(&self, name: &str) -> Option<&str>;
    fn query(&self, name: &str) -> Option<&str>;
}
