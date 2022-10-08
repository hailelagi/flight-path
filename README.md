# Flight

A flight path microservice API.

## Api Spec

Responds to json-ecoded `POST` requests on the `./calculate` route. This _must_ include an array of at least one flight path or a `422 - Unprocessable Entity is returned`. For example:

```bash
curl -XPOST -H "Content-type: application/json" -d '[["SFO", "EWR"]]'  'localhost:8080/calculate'
curl -XPOST -H "Content-type: application/json" -d '[["ATL", "EWR"], ["SFO", "ATL"]]'  'localhost:8080/calculate'
curl -XPOST -H "Content-type: application/json" -d '[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]'  'localhost:8080/calculate'
```

## Schema

```json
[['SFO', 'EWR']]  => ['SFO', 'EWR']
[['ATL', 'EWR'], ['SFO', 'ATL']]     => ['SFO', 'EWR']
[['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] => ['SFO', 'EWR']
```

## Installation

Assuming you have the rust toolchain installed with `cargo`: `cargo run`
Testing: `cargo test`

## Estimation

My rust is only so-so I recently started getting serious with it about a week or two ago, I have never used rust on the web.
Took about six hours. Including tests and documentation. The solution itself was not hard to come up with, navigating some of the idiosyncracies of
actix took up most of the time.
