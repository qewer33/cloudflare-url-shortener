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
            let html404 = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>404</title>
            </head>
            <body>
                <h1>Seems like this short URL doesn't exit anymore...</h1>
                <a href="https://qewer.dev">CLICK HERE TO GO TO MY HOMEPAGE</a>
            </body>
            </html>
            <style>
                body {
                    height: 100vh;
                    display: flex;
                    flex-direction: column;
                    font-family: Arial, sans-serif;
                    background-color: #1D1D1D;
                    justify-content: center;
                    align-items: center;
                }
                h1 {
                    color: #FFFFFF;
                }
                a {
                    color: #FFFFFF;
                    background-color: #0578F0;
                    transform: skew(-12deg);
                    padding: 15px 30px;
                    text-decoration: none;
                    font-weight: 900;
                }
            </style>
            "#;
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
        .get("/", |_, _| {
            Response::from_html("Hello from Qewer's URL shortener!")
        })
        .get_async("/:key", UrlShortener::redirect)
        .post_async("/create/:secret", UrlShortener::create)
        .post_async("/delete/:secret", UrlShortener::delete)
        .run(req, env)
        .await
}
