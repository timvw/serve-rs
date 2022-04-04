# serve-rs

Explore messaging in Rust.

```bash
grpcurl -plaintext -import-path ./proto -proto broker.proto -d "{ \"message\": \"tim $(date)\"}" [::]:50051 broker.Broker/Publish



grpcurl -plaintext -import-path ./proto -proto broker.proto [::]:50051 broker.Broker/Subscribe


grpcurl -vv -plaintext -import-path ./proto -proto broker.proto [::]:50051 broker.Broker/Subscribe
```