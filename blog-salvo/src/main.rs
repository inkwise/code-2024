use std::fs::File;
use std::io::Read;

use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use salvo::routing::path;

#[handler]
async fn getMD(req: &mut Request,res: &mut Response){
    let path = req.param::<&str>("name").expect("读取参数失败");
    let str=read_file(path);
    res.render(str.await);
    
}
#[handler]
async fn hello(res: &mut Response) {
    let st = read_file("1.md");
    res.render(st.await);
}
async fn read_file(path:  &str) -> String {
    let xx = format!("{}{}","./blog/",path);
    let mut file = File::open(xx).expect("读取文件失败!");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("读取文件失败2");
    contents
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cors = Cors::new()
        .allow_origin("https://salvo.rs")
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();
    let router = Router::new().get(hello).push(
        Router::with_path("<name>").get(getMD)
    );
    let service = Service::new(router).hoop(cors);

    let acceptor = TcpListener::new("0.0.0.0:10000").bind().await;
    Server::new(acceptor).serve(service).await;
}
