use serde::Deserialize;
use worker::*;

mod utils;

#[derive(Deserialize, Debug)]
struct CreateRequest {
    key: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct DeleteRequest {
    key: String,
}

struct UrlShortener;

impl UrlShortener {
    fn serve_index(_: Request, _: RouteContext<()>) -> Result<Response> {
        let html = include_str!("index.html");
        Response::from_html(html)
    }

    async fn redirect(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        if let Some(key) = ctx.param("key") {
            let url = ctx.kv("KV")?.get(key.trim()).text().await;

            if let Ok(Some(url)) = url {
                let html = r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Redirecting...</title>
                    <link rel="canonical" href="${url}">
                    <script>location = "${url}"</script>
                    <meta http-equiv="refresh" content="0;url=${url}">
                    <meta name="robots" content="noindex">
                </head>
                <body>
                    <h1>Redirecting...</h1>
                    <a href="${url}">Click here if you are not redirected.</a>
                </body>
                </html> 
                "#
                .replace("${url}", url.as_str());
                return Response::from_html(html);
            }
            let html404 = include_str!("404.html");
            return Response::from_html(html404);
        }
        Response::error("Bad Request: Missing Parameter", 400)
    }

    async fn create(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        if let Some(secret) = ctx.param("secret") {
            if *secret == ctx.secret("SECRET")?.to_string() {
                let req_body: Result<CreateRequest> = req.json().await;

                if let Ok(req_body) = req_body {
                    let kv_result = ctx.kv("KV")?.put(&req_body.key, req_body.url);

                    if let Ok(kv_result) = kv_result {
                        let kv_result = kv_result.execute().await;

                        if kv_result.is_ok() {
                            return Response::from_html(format!(
                                "Url Successfully Shortened! You can access it at: s.qewer.dev/{}",
                                &req_body.key
                            ));
                        }
                        return Response::error("Bad Request: KV Operation Failed", 400);
                    }
                    return Response::error("Bad Request: KV Operation Failed", 400);
                }
                return Response::error("Bad Request: Wrong Body", 400);
            }
            return Response::error("Bad Request: Wrong Secret", 400);
        }
        Response::error("Bad Request: Missing Secret", 400)
    }

    async fn delete(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
        if let Some(secret) = ctx.param("secret") {
            if *secret == ctx.secret("SECRET")?.to_string() {
                let req_body: Result<DeleteRequest> = req.json().await;

                if let Ok(req_body) = req_body {
                    let kv_result = ctx.kv("KV")?.delete(&req_body.key).await;

                    if let Ok(kv_result) = kv_result {
                        return Response::from_html("Key Successfully Deleted");
                    }
                    return Response::error("Bad Request: KV Operation Failed", 400);
                }
                return Response::error("Bad Request: Wrong Body", 400);
            }
            return Response::error("Bad Request: Wrong Secret", 400);
        }
        Response::error("Bad Request: Missing Secret", 400)
    }
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", UrlShortener::serve_index)
        .get_async("/:key", UrlShortener::redirect)
        .post_async("/create/:secret", UrlShortener::create)
        .post_async("/delete/:secret", UrlShortener::delete)
        .run(req, env)
        .await
}
