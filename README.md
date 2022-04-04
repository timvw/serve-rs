# serve-rs

Explore messaging in Rust.

```bash
grpcurl -plaintext -import-path ./proto -proto server.proto -d "{ \"message\": \"tim $(date)\"}" [::]:50051 server.Publisher/Publish



grpcurl -plaintext -import-path ./proto -proto server.proto [::]:50051 server.Publisher/Subscribe


grpcurl -vv -plaintext -import-path ./proto -proto server.proto [::]:50051 server.Publisher/Subscribe
```