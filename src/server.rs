use {
    crate::{asset, HTTPRequest, Request, Response, Result, Router},
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;

pub fn run<T: ToSocketAddrs, R: 'static + HTTPRequest>(addr: T, router: Router<R>) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;
    let server = Arc::new(Server::new(router));
    println!("~ vial running at http://{}", addr);

    for stream in listener.incoming() {
        let server = server.clone();
        let stream = stream?;
        pool.execute(move || {
            if let Err(e) = server.handle_request(stream) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

pub struct Server<R: HTTPRequest> {
    router: Router<R>,
}

impl<R: HTTPRequest> Server<R> {
    pub fn new(router: Router<R>) -> Server<R> {
        Server { router }
    }

    fn handle_request(&self, mut stream: TcpStream) -> Result<()> {
        let reader = stream.try_clone()?;
        let req = R::from_reader(reader)?;
        self.write_response(stream, req)
    }

    fn write_response(&self, mut stream: TcpStream, mut req: R) -> Result<()> {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let mut response = self.build_response(req);

        println!("{} {} {}", method, response.code, path);
        if response.code == 500 {
            eprintln!("{}", response.body);
        }

        response.write(stream)
    }

    fn build_response(&self, mut req: R) -> Response {
        if asset::exists(req.path()) {
            if let Some(req_etag) = req.header("If-None-Match") {
                if req_etag == asset::etag(req.path()).as_ref() {
                    Response::from(304)
                } else {
                    Response::from_asset(req.path())
                }
            } else {
                Response::from_asset(req.path())
            }
        } else if let Some(action) = self.router.action_for(&mut req) {
            action(req)
        } else {
            Response::from(404)
        }
    }
}
