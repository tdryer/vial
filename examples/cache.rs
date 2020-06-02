use vial::prelude::*;

routes! {
    GET "/" => hello;
}

struct Cache {
    count: usize,
}

fn hello(req: Request) -> impl Responder {
    let cache = req.cache::<Cache>();
    format!("Hits: {}", count(cache))
}

fn count(cache: std::rc::Rc<Cache>) -> String {
    cache.count += 1;
    cache.count.to_string()
}

fn main() {
    vial::with_cache!(Cache { count: 0 });
    run!().unwrap()
}
