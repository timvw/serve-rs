
Build container

```bash
docker build . -f deploy/Dockerfile -t broker:latest
```

Run container:

```bash
docker run --rm -it -p 50051:50051 broker:latest
```

Publish container

```bash
docker tag broker:latest registry.apps.timvw.be/broker:latest
docker push registry.apps.timvw.be/broker:latest
```

Forward port:

```bash
 kubectl port-forward service/broker -n broker 50051:50051
 ```

Cleanup: 

```bash
kubectl delete all -l app=broker -n broker
kubectl delete ingress -l app=broker -n broker
```


Call publish method

```bash
grpcurl -plaintext -import-path ./proto -proto broker.proto -d '{"message": "hi"}' '[::1]:50051'  broker.Broker/Publish
```

Run performance test:

```bash
ghz -c 100 -n 100000 --insecure \
--proto ./proto/broker.proto \
--call broker.Broker/Publish \
-d '{"message": "hi"}' \
localhost:50051
```