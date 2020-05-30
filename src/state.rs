use {
    crate::{HTTPRequest, Request, Result},
    std::net::TcpStream,
};

pub struct State<T> {
    inner: T,
    request: Request,
}

impl<T> std::ops::Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> HTTPRequest for State<T> {
    fn method(&self) -> &str {
        self.request.method()
    }
    fn path(&self) -> &str {
        self.request.path()
    }
    fn full_path(&self) -> &str {
        self.request.full_path()
    }
    fn body(&self) -> &str {
        self.request.body()
    }
    fn header(&self, key: &str) -> Option<&str> {
        self.request.header(key)
    }
    fn arg(&self, key: &str) -> Option<&str> {
        self.request.arg(key)
    }
    fn form(&self, key: &str) -> Option<&str> {
        self.request.form(key)
    }
    fn query(&self, key: &str) -> Option<&str> {
        self.request.query(key)
    }
    fn set_arg(&mut self, key: String, value: String) {
        self.request.set_arg(key, value)
    }
    fn from_reader(reader: TcpStream) -> Result<Self> {
        State {
            inner: T::new(),
            Self::from_reader(reader)
    }
}
