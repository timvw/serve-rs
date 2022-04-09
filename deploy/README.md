
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