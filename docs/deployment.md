# Deployment

How to deploy StrataRouter in production environments.

---

## Docker (Standalone)

Build and run the StrataRouter container:

```bash
# Build
docker build -t stratarouter:latest .

# Run with default settings
docker run -p 8000:8000 stratarouter:latest

# Run with environment variables
docker run -p 8000:8000 \
  -e OPENAI_API_KEY=sk-... \
  -e SR_LOG_LEVEL=info \
  stratarouter:latest
```

---

## Docker Compose

```bash
# Start all services (StrataRouter + Prometheus + Grafana)
docker-compose up -d

# View logs
docker-compose logs -f stratarouter

# Stop
docker-compose down
```

Services exposed:
- `http://localhost:8000` — StrataRouter API
- `http://localhost:9090` — Prometheus
- `http://localhost:3000` — Grafana

---

## Kubernetes

Basic Deployment manifest:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: stratarouter
spec:
  replicas: 3
  selector:
    matchLabels:
      app: stratarouter
  template:
    metadata:
      labels:
        app: stratarouter
    spec:
      containers:
        - name: stratarouter
          image: ghcr.io/ai-deeptech/stratarouter:latest
          ports:
            - containerPort: 8000
          env:
            - name: SR_LOG_LEVEL
              value: "info"
          resources:
            requests:
              memory: "128Mi"
              cpu: "250m"
            limits:
              memory: "512Mi"
              cpu: "1000m"
          readinessProbe:
            httpGet:
              path: /health
              port: 8000
            initialDelaySeconds: 5
            periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: stratarouter
spec:
  selector:
    app: stratarouter
  ports:
    - port: 80
      targetPort: 8000
  type: ClusterIP
```

Apply:

```bash
kubectl apply -f k8s/deployment.yaml
kubectl rollout status deployment/stratarouter
```

---

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `SR_LOG_LEVEL` | `info` | Log level: `debug`, `info`, `warn`, `error` |
| `SR_PORT` | `8000` | HTTP server port |
| `SR_WORKERS` | `4` | Number of worker threads |
| `SR_CACHE_SIZE` | `1000` | LRU embedding cache capacity |
| `SR_CACHE_TTL` | `300` | Cache TTL in seconds |
| `OPENAI_API_KEY` | — | Required for OpenAI encoder |
| `COHERE_API_KEY` | — | Required for Cohere encoder |

---

## Health Check

```bash
curl http://localhost:8000/health
# {"status": "ok", "version": "0.2.1"}
```

---

## Prometheus Metrics

StrataRouter exposes Prometheus metrics at `/metrics`:

| Metric | Type | Description |
|---|---|---|
| `stratarouter_requests_total` | Counter | Total routing requests |
| `stratarouter_latency_seconds` | Histogram | Routing latency |
| `stratarouter_cache_hits_total` | Counter | Embedding cache hits |
| `stratarouter_no_match_total` | Counter | Queries with no route match |

---

## Production Checklist

- [ ] Set memory limits (`ulimits` or K8s `resources.limits`)
- [ ] Enable `readinessProbe` before adding to load balancer
- [ ] Configure `SR_CACHE_SIZE` based on your query volume
- [ ] Mount API keys as K8s Secrets, not env vars in manifests
- [ ] Enable Prometheus scraping for latency and error monitoring
- [ ] Pin image tag to a specific version (not `latest`) for reproducibility

---

## StrataRouter Runtime

For production deployments with TCFP execution, semantic cache, REST API,
and Prometheus metrics, see the Runtime:

→ [stratarouter-runtime](https://github.com/ai-deeptech/stratarouter-runtime)

---

## Enterprise Deployment

For multi-tenant, SOC 2 / HIPAA-compliant, audited deployments:

→ **[support@stratarouter.com](mailto:support@stratarouter.com)**  
→ **[stratarouter.com](https://stratarouter.com)**
