# MockServer 🦀

Um servidor mock altamente configurável escrito em Rust para simular APIs durante desenvolvimento e testes.

## Features

- 🎯 **Endpoints Dinâmicos** - Configure via YAML/JSON
- 🎲 **Dados Fake** - Geração automática com 25+ tipos (nomes, emails, UUIDs, etc.)
- ⏱️ **Delay/Timeout** - Simule latência e timeouts
- ✅ **Validação** - Valide requests com JSON Schema
- 🔀 **Lógica Condicional** - Respostas diferentes baseadas em params/headers

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

Edite `config/endpoints.yaml`:

```yaml
server:
  port: 3000

endpoints:
  # Endpoint com dados fake
  - path: "/api/users"
    method: GET
    response:
      status: 200
      body:
        users:
          $array:
            count: 5
            template:
              id: { $fake: "uuid" }
              name: { $fake: "name" }
              email: { $fake: "email" }

  # Endpoint com delay
  - path: "/api/slow"
    method: GET
    delay: 3000
    response:
      status: 200
      body: { message: "Delayed response" }

  # Endpoint com condição
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

## API Especial

- `$fake`: Gera dado fake
- `$param`: Valor do path parameter
- `$body`: Valor do request body
- `$array`: Gera array com template

## License

MIT
