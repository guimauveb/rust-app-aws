## Notes

**Code that goes along with [this article](https://guimauve.io/articles/50)**

This is a complete article about how to deploy a Rust web application on an AWS EC2 instance using a classic backend/frontend structure. It's derived from my own experience building [my own website](https://github.com/guimauveb/guimauve.io) using the same tools and having a great time doing so.

This application is made of a WebAssembly frontend displaying articles retrieved from an API. The purpose of the article is to show how to build and deploy a minimal yet complete full stack Rust web application from a local machine to a web server.

## Frameworks used

- Backend
    - Actix
    - Postgres
    - Diesel

- Frontend
    - Yew


## Running the application

### Backend
#### From `backend/`
`cargo run`

or if you want hot reload with `cargo-watch`:

`cargo watch -x 'run backend`

### Frontend
#### From `frontend/`
`trunk serve`
