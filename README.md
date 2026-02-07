# MockServer 🦀

A highly configurable mock server written in Rust for simulating APIs during development and testing.

## Features

- 🎯 **Dynamic Endpoints** - Configure via YAML or HTTP
- 🎲 **Fake Data** - Automatic generation with 25+ types (names, emails, UUIDs, etc.)
- ⏱️ **Delay/Timeout** - Simulate latency and timeouts
- ✅ **Validation** - Validate requests with JSON Schema
- 🔀 **Conditional Logic** - Different responses based on params/headers
- 🔄 **Dynamic Configuration** - Update endpoints via HTTP without restart

## Quick Start

```bash
# Build
cargo build --release

# Run
cargo run

# Or with custom config
CONFIG_PATH=./my-config.yaml cargo run
```

Server starts at `http://localhost:3000`

## Configuration

### Via YAML File

Edit `config/endpoints.yaml`:

```yaml
server:
  host: "0.0.0.0"
  port: 3000

endpoints:
  - path: "/health"
    method: GET
    response:
      status: 200
      body:
        status: "ok"
        timestamp: { $fake: "datetime" }
```

### Via HTTP

Update configuration dynamically:

```bash
# View current configuration
curl http://localhost:3000/_config

# Update configuration
curl -X POST http://localhost:3000/_config \
  -H "Content-Type: application/json" \
  -d '{
    "endpoints": [
      {
        "path": "/api/users",
        "method": "GET",
        "response": {
          "status": 200,
          "body": {
            "users": {
              "$array": {
                "count": 5,
                "template": {
                  "id": { "$fake": "uuid" },
                  "name": { "$fake": "name" },
                  "email": { "$fake": "email" }
                }
              }
            }
          }
        }
      }
    ]
  }'
```

## Configuration Examples

### Endpoint with Delay
```yaml
- path: "/api/slow"
  method: GET
  delay: 3000  # 3 seconds
  response:
    status: 200
    body: { message: "Delayed response" }
```

### Endpoint with Path Params
```yaml
- path: "/api/users/:id"
  method: GET
  response:
    status: 200
    body:
      id: { $param: "id" }
      name: { $fake: "name" }
```

### Endpoint with Conditions
```yaml
- path: "/api/users/:id"
  method: GET
  response:
    body:
      id: { $param: "id" }
      name: { $fake: "name" }
  conditions:
    - if: { param: "id", equals: "999" }
      response:
        status: 404
        body: { error: "Not found" }
```

### Endpoint with Random Delay
```yaml
- path: "/api/random-delay"
  method: GET
  delay:
    type: random
    min: 100
    max: 500
  response:
    status: 200
    body: { message: "Random delay" }
```

## Fake Data Types

| Type | Description |
|------|-------------|
| `uuid` | UUID v4 |
| `name` | Full name |
| `email` | Email address |
| `phone` | Phone number |
| `number` | Number (with min/max) |
| `datetime` | ISO datetime |
| `address` | Full address |
| `company` | Company name |
| `sentence` | Lorem ipsum sentence |
| `url` | URL |
| `boolean` | true/false |
| `color` | Hexadecimal color |

## Special Directives

- `$fake` - Generate fake data
- `$param` - Path parameter value
- `$body` - Request body value
- `$array` - Generate array with template

## Management Endpoint

### `GET /_config`
Returns the current server configuration.

### `POST /_config`
Updates the server configuration dynamically.

## License

MIT
