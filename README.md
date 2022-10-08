# Flight Path

A flight path microservice API.

## Api Spec

Responds to json-ecoded `POST` requests on the `./calculate` route. This _must_ include an array of at least one flight path or a `422 - Unprocessable Entity is returned`. For example:

```bash
curl -H "Content-type: application/json" -d '[['ATL', 'EWR'], ['SFO', 'ATL']]' 'http://localhost:8080/calculate'
```

## Schema

```
[['SFO''EWR']]  => ['SFO', 'EWR']
[['ATL', 'EWR'], ['SFO', 'ATL']]     => ['SFO', 'EWR']
[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] => ['SFO', 'EWR']

```

## Installation

Assuming you have the rust toolchain installed with `cargo`: `cargo run`

## Estimation

My rust is only so-so I recently started getting serious with it about a week or two ago, I have never used rust on the web, I had to learn actix, futures/async and ownership/lifetime of a graph data structure in rust on the fly.
