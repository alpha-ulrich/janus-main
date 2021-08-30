use warp::Filter;

/*
 * Currently just returning 200 OK with body "Hello name", on GET /hello/name
 */
#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
                .map(|name| format!("Hello, {}", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
