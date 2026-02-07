# MockServer 🦀

Um servidor mock altamente configurável escrito em Rust para simular APIs durante desenvolvimento e testes.

## Features

- 🎯 **Endpoints Dinâmicos** - Configure via YAML ou HTTP
- 🎲 **Dados Fake** - Geração automática com 25+ tipos (nomes, emails, UUIDs, etc.)
- ⏱️ **Delay/Timeout** - Simule latência e timeouts
- ✅ **Validação** - Valide requests com JSON Schema
- 🔀 **Lógica Condicional** - Respostas diferentes baseadas em params/headers
- 🔄 **Configuração Dinâmica** - Atualize endpoints via HTTP sem reiniciar

## Quick Start

```bash
# Build
cargo build --release

# Executar
cargo run

# Ou com config customizada
CONFIG_PATH=./my-config.yaml cargo run
```

O servidor inicia em `http://localhost:3000`

## Configuração

### Via Arquivo YAML

Edite `config/endpoints.yaml`:

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

Atualize a configuração dinamicamente:

```bash
# Ver configuração atual
curl http://localhost:3000/_config

# Atualizar configuração
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

## Exemplos de Configuração

### Endpoint com Delay
```yaml
- path: "/api/slow"
  method: GET
  delay: 3000  # 3 segundos
  response:
    status: 200
    body: { message: "Delayed response" }
```

### Endpoint com Path Params
```yaml
- path: "/api/users/:id"
  method: GET
  response:
    status: 200
    body:
      id: { $param: "id" }
      name: { $fake: "name" }
```

### Endpoint com Condições
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

### Endpoint com Delay Randômico
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

## Tipos de Dados Fake

| Tipo | Descrição |
|------|-----------|
| `uuid` | UUID v4 |
| `name` | Nome completo |
| `email` | Email |
| `phone` | Telefone |
| `number` | Número (com min/max) |
| `datetime` | Data/hora ISO |
| `address` | Endereço |
| `company` | Nome de empresa |
| `sentence` | Frase lorem |
| `url` | URL |
| `boolean` | true/false |
| `color` | Cor hexadecimal |

## Diretivas Especiais

- `$fake` - Gera dado fake
- `$param` - Valor do path parameter
- `$body` - Valor do request body
- `$array` - Gera array com template

## Endpoint de Gerenciamento

### `GET /_config`
Retorna a configuração atual do servidor.

### `POST /_config`
Atualiza a configuração do servidor dinamicamente.

## License

MIT
