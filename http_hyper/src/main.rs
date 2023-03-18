use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::net::SocketAddr;

async fn handle_get(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 检查请求方法是否为GET
    if req.method() != Method::GET {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap());
    }

    // 设置响应体
    let body = "Hello, World!";
    Ok(Response::new(Body::from(body)))
}

async fn handle_post(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 检查请求方法是否为POST
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap());
    }

    // 读取请求体
    let full_body = hyper::body::to_bytes(req.into_body()).await?;
    let body_str = String::from_utf8_lossy(&full_body).into_owned();

    // 设置响应体
    let body = format!("Received body: {}", body_str);
    Ok(Response::new(Body::from(body)))
}


// #[tokio::main] 是一个 Rust 的属性（attribute），它告诉 Rust 编译器这个程序应该使用 Tokio 运行时（runtime）来运行。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 5329));
    let make_svc = hyper::service::make_service_fn(|_conn| async {
        // 第一个类型参数是占位符（Rust 中的匿名类型）,,,第二个类型参数是 hyper::Error 类型，用于表示操作可能抛出的错误类型
        // 这是 Hyper 库提供的一个函数，可以将一个闭包（closure）转换为一个实现了 Service trait 的服务实例。该闭包接收一个 Request 参数，返回一个实现了 Future<Output = Result<Response<Body>, E>> trait 的 future。
        Ok::<_, hyper::Error>(hyper::service::service_fn(|req| async move {
            match (req.method(), req.uri().path()) {
                // 处理GET请求
                (&Method::GET, "/") => handle_get(req).await,
                // 处理POST请求
                (&Method::POST, "/") => handle_post(req).await,
                // 处理其他请求
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap()),
            }
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    server.await?;
    Ok(())
}
