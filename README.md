## Hopper
Hopper is a URL shortener written in rust. Nothing major - primarily to learn rust.

Very simple architecture.

MongoDB with a single document for storing a list of `aliases` -> `original_url`

REST API which sends `301` redirects to map your requests `/alias` to the original url

#### TODO:

- Docker setup
- POST endpoint to add new URL aliases
- UI
- SaaS integration (??)
