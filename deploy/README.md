
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

docker tag broker:latest docker.io/timvw/broker:latest
docker push docker.io/timvw/broker:latest
```

(Re-)deploy

```bash
kubectl rollout restart -n broker deployment/broker
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


grpcurl -vv -plaintext -import-path ./proto -proto broker.proto -d '{"message": "hi"}' 'broker.apps.timvw.be:50051'  broker.Broker/Publish


```

Run performance test:

```bash
ghz -c 100 -n 1000000 --insecure \
--proto ./proto/broker.proto \
--call broker.Broker/Publish \
-d '{"message": "hi"}' \
'[::1]:50051'
```

With a proxy tunnel to my k8s machine:

```
Summary:
Count:        1000000
Total:        83.72 s
Slowest:      179.88 ms
Fastest:      2.48 ms
Average:      7.55 ms
Requests/sec: 11944.96

Response time histogram:
2.476   [1]      |
20.216  [996826] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
37.957  [970]    |
55.698  [147]    |
73.439  [156]    |
91.180  [0]      |
108.921 [0]      |
126.662 [0]      |
144.403 [809]    |
162.144 [791]    |
179.885 [300]    |

Latency distribution:
10 % in 5.48 ms
25 % in 6.22 ms
50 % in 7.09 ms
75 % in 8.07 ms
90 % in 9.11 ms
95 % in 10.02 ms
99 % in 13.74 ms

Status code distribution:
[OK]   1000000 responses 
```

Get k8s dashboard token

```bash
kubectl get secret -n kube-system $(kubectl get secret -n kube-system | grep kubernetes-dashboard-token | awk '{print $1}') -o json | jq -r .data.token | base64  -d | pbcopy
```

Allowing access to GRPC/TCP port in microk8s

```bash
kubectl patch cm nginx-ingress-tcp-microk8s-conf -n ingress --patch '{"data":{"50001":"broker/broker:50001"}}'
kubectl patch ds nginx-ingress-microk8s-controller --patch "$(cat ./deploy/ingress.yml)" -n ingress
```


