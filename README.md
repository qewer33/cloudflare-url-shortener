# Qewer's URL Shortener

A simple URL shortener built with Rust and Cloudflate Workers. It has four routes:

`/`: Shows message
`/:key`: This is the main route for shortened URLs. It responds with a redirect HTML page if the key is in the KV, shows a 404 page otherwise
`/create/:secret`: Adds a new key-value combination to the KV (request body should have a "key" and a "url" field), needs secret as a parameter
`/delete/:secret`: Deletes a key-value combination from the KV by key (request body should have a "key" field), needs secret as a parameter

```bash
# condifgure the SECRET
wrangler secret put SECRET

# compiles project to WebAssembly
wrangler build

# runs the Worker locally
wrangler dev

# deploys the Worker to Cloudflare
wrangler publish
```
