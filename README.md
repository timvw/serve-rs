# serve-rs

Explore messaging in Rust.

```bash
grpcurl -plaintext -import-path ./proto -proto server.proto -d '{ "message": "t"}' [::]:50051 server.Publisher/Publish
```