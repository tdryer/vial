use {
    crate::{asset, HTTPRequest, Request, Response, Result, Router, State},
    std::{
        io::{self, prelude::*, BufReader, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
        sync::{Arc, Mutex},
    },
    threadpool::ThreadPool,
};

const MAX_CONNECTIONS: usize = 10;

pub fn run_with_state<T: ToSocketAddrs, R: 'static + HTTPRequest, S: 'static + Send + Sync>(
    addr: T,
    router: Router<R>,
    state: Option<S>,
) -> Result<()> {
    let pool = ThreadPool::new(MAX_CONNECTIONS);
    let listener = TcpListener::bind(&addr)?;
    let addr = listener.local_addr()?;
    let server = Arc::new(Server::new(router));
    let state = Arc::new(state);
    println!("~ vial running at http://{}", addr);

    for stream in listener.incoming() {
        let server = server.clone();
        let state = state.clone();
        let stream = stream?;
        pool.execute(move || {
            if let Err(e) = server.handle_request(stream, state) {
                eprintln!("!! {}", e);
            }
        });
    }

    Ok(())
}

pub fn run<T: ToSocketAddrs>(addr: T, router: Router<Request>) -> Result<()> {
    run_with_state::<_, _, Request>(addr, router, None)
}

pub struct Server<R: HTTPRequest> {
    router: Router<R>,
}

impl<R: HTTPRequest + 'static> Server<R> {
    pub fn new(router: Router<R>) -> Server<R> {
        Server { router }
    }

    fn handle_request<S: 'static + Send + Sync>(
        &self,
        mut stream: TcpStream,
        state: Arc<Option<S>>,
    ) -> Result<()> {
        let reader = stream.try_clone()?;
        let req = Request::from_reader(reader)?;
        let req: Box<dyn HTTPRequest> = if state.is_some() {
            Box::new(State::new(state.unwrap(), req))
        } else {
            Box::new(req)
        };
        self.write_response(stream, req)
    }

    fn write_response(&self, mut stream: TcpStream, mut req: Box<dyn HTTPRequest>) -> Result<()> {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let mut response = self.build_response(req);

        println!("{} {} {}", method, response.code, path);
        if response.code == 500 {
            eprintln!("{}", response.body);
        }

        response.write(stream)
    }

    fn build_response(&self, mut req: Box<dyn HTTPRequest>) -> Response {
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
            action(*req)
        } else {
            Response::from(404)
        }
    }
}
