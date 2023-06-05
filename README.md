# ERP FACULDADE RUST ACTIX

Um REST server em actix 2.0 na linguagem rust

## Motivação

Aplicações feitas em rust são mais rapidas e tem maior segurança de memoria,
fiz o projeto em rust para aprender um web framework backend da linguagem

## Essa API possui

- Actix 2.x HTTP Server
- Postgres
- JWT Support
- Camada de Caching Assincrona
- Serviço público e seguro de caching de static files
- As operações de banco do diesel sem blocking (utiliza multithreading para utilizar multiplas conexões)
- Sistema de arquivos feito para escalar
- .env para desenvolvimento local
- Integração de estado da aplicação com uma api simples
- struct de Lazy Static Config
- Built-in Healthcheck
- Listeners configurados para TDD
- Erros customizados na validação de HTTP Payload/Json
- Hashing seguro de senha Argon2i
- Suporte a CORS
- Testes unitários e de integração
- Reports dos testes
- Dockerfile para rodar dentro de um server em um container
- Integração com tarvisCI



## Como rodar

Clone o repósitorio e entre dentro da pasta pelo terminal

```shell
git clone https://github.com/NikolasSilvaLunkes/erp-faculdade-rust.git
cd rust-actix-example
```

Configure no .env o banco de dados

Instale o diesel_cli

```shell
cargo install diesel_cli --no-default-features --features postgres
```

```shell
diesel migration run
```

## Rodando num servidor

```shell
cargo run
```

## Autoreloading

Para dar autoreload quando mudar o codigo:

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

## Testes

Os testes estão na pasta `/src/tests`.

### Rodando os testes

Para rodar todos os testes

```shell
cargo test
```

