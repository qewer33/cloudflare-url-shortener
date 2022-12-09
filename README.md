# Qewer's URL Shortener

A simple URL shortener built with Rust and Cloudflare Workers. It includes a fairly straightforward web UI that uses my main website's design style.

Check out the URL shortener at https://s.qewer.dev and my main website at https://qewer.dev

![screenshot_1](https://github.com/qewer33/cloudflare-url-shortener/blob/main/assets/screenshot.png?raw=true)

It has four routes:

`/`: Shows create/delete user interface

`/:key`: This is the main route for shortened URLs. It responds with a redirect HTML page if the key is in the KV, shows a 404 page otherwise

`/create/:secret`: Adds a new key-value combination to the KV (request body should have a "key" and a "url" field), requires secret as a parameter

`/delete/:secret`: Deletes a key-value combination from the KV by key (request body should have a "key" field), requires secret as a parameter

```bash
# configure the SECRET
wrangler secret put SECRET

# compiles project to WebAssembly
wrangler build

# runs the Worker locally
wrangler dev

# deploys the Worker to Cloudflare
wrangler publish
```
