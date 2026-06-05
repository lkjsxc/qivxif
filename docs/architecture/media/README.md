# Media Architecture

## Contents

- [storage.md](storage.md): browser and server storage contracts.
- [http.md](http.md): upload and serving routes.

## Ownership

Browser repositories own local metadata and OPFS chunk locators. The optional
server owns authenticated upload, durable blob storage, range serving, public
routes, and ACL enforcement.
