
## Config
Env var:
* PRISMA_ACCESS_KEY
* PRISMA_SECRET_KEY
* PRISMA_API_ENDPOINT

example of `config.json`
```
{
  "query": {
    "webClient": false,
    "detailed": false,
    "filters": [
      {
        "name": "timeRange.type",
        "operator": "=",
        "value": "ALERT_OPENED"
      },
      {
        "operator": "=",
        "name": "alert.status",
        "value": "open"
      },
      {
        "operator": "=",
        "name": "policy.severity",
        "value": "high"
      },
      {
        "operator": "=",
        "value": "anomaly",
        "name": "policy.type"
      },
      {
        "operator": "=",
        "name": "policy.type",
        "value": "attack_path"
      },
      {
        "value": "audit_event",
        "operator": "=",
        "name": "policy.type"
      },
      {
        "name": "policy.type",
        "value": "network",
        "operator": "="
      },
      {
        "value": "config",
        "name": "policy.type",
        "operator": "="
      },
      {
        "value": "workload_vulnerability",
        "name": "policy.type",
        "operator": "="
      },
      {
        "name": "policy.type",
        "operator": "=",
        "value": "workload_incident"
      }
    ],
    "timeRange": {
      "type": "relative",
      "value": {
        "amount": "24",
        "unit": "hour"
      }
    }
  },
  "alerts": []
}
```