# Setup Instructions

## Setup for Development

1. Install `cargo` and `rustup` from your favorite package manager. If
   developing in a text editor which supports [LSP](https://microsoft.github.io/language-server-protocol/),
   install the `rust-analyzer` binary to greatly improve the development experience. The easiest version of
   this is in VSCode, where you can simply install `rust-analyzer` as an extension.
2. Run `rustup toolchain stable` to install the most recent stable version of Rust.
3. Install [Docker](https://www.docker.com/products/docker-desktop/).
4. This project uses `sqlx`, a library which optionally validates code against
   a running SQL database. To have the "full" development experience, it is
   recommended to follow the rest of the instructions. To work in offline mode,
   `export SQLX_OFFLINE=true` then compile and run with `cargo run`.
5. To start *only* the Postgres Docker container, run `docker compose -f ./docker-compose-dev.yml up`.
6. You can now compile the project with `cargo check`, or run with `cargo run`.

*Important:*
When actually developing code in this repository and `sqlx` queries or database schemas are changed,
you must run `cargo sqlx prepare`. This command regenerates the `sqlx-data.json` file, which should be checked in 
and committed -- this file is used when compiling offline and building the project's Docker iamge.

## Setup for Testing and Experimentation

If you simply wish to run the `dbio-protocol` server locally for API testing,
there is a `docker-compose.yml` which connects the most recent code (in this repository branch)
with a Postgres database. You can simply run `docker compose up` or
`docker compose up --detach` to spin up both the protocol and database
containers. The whole setup should take about 5 minutes to build and run on a modern Macbook.
One running, the protocol server will bind to `localhost:8080`.

# API Documentation

## Routes

[Users](#users)

- [POST /dbio/users](#post-dbiousers)
- [GET /dbio/users/eth/{eth_address}](#get-dbiousersethethaddress)
- [GET /dbio/users/email/{email}](#get-dbiousersemailemail)

[Resources](#resources)

- [POST /dbio/resources](#post-dbioresources)
- [GET /dbio/resources/{subject_eth_address}](#get-dbioresourcessubjectethaddress)

### Users

#### `POST /dbio/users`
The post request to `/dbio/users` requires a JSON payload that represents a user to be sent in the body.
```json
{
	"eth_public_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
	"email": "user@example.com"
}
```
The parameters in the JSON payload are:
- `eth_public_address: String` - the Ethereum public address of the user.
- `email: String` - the email address of the user.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The user object was created.
- `409 Conflict` - The user object already exists.

#### `GET /dbio/users/eth/{eth_address}`
The get request to `/dbio/users/eth/{eth_address}` takes as path parameters the following items:
- `eth_address` - The Ethereum public address of the user being queried.

The response returned is one of the following:
- `200 Ok` - The user was found.
- `404 Not Found` - The user was not found.

In the case of `200 Ok`, the body of the response contains JSON.
```json
{
	"eth_public_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
	"email": "user@example.com"
}
```
The JSON returned contains the following information:
- `eth_public_address: String` - the Ethereum public address of the user.
- `email: String` - the email address of the user.

#### `GET /dbio/users/email/{email}`
The get request to `/dbio/users/email/{email}` takes as path parameters the following items:
- `email` - The email address of the user being queried.

The response returned is one of the following:
- `200 Ok` - The user was found.
- `404 Not Found` - The user was not found.

In the case of `200 Ok`, the body of the response contains JSON.
```json
{
	"eth_public_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
	"email": "user@example.com"
}
```
The JSON returned contains the following information:
- `eth_public_address: String` - the Ethereum public address of the user.
- `email: String` - the email address of the user.

### Resources

#### `POST /dbio/resources`
The post request to `/dbio/resources` requires a JSON payload that represents a user to be sent in the body.
```json
{
	"email": "user@example.com",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "resource_type": "insert_resource_type_here",
    "resource_id": 1,
    "ciphertext": "insert_ciphertext_here"
}
```
The parameters in the JSON payload are:
- `email: String` - the email address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the entity submitting the resource.
- `resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `resource_id: Integer` - the ID of the resource in the submitter's system.
- `ciphertext: String` - the ciphertext generated after encrypting the resource.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The user object was created.
- `409 Conflict` - A resource with the same `creator_eth_address` and `resource_id` pair already exists.

#### `GET /dbio/resources/{subject_eth_address}`
The get request to `/dbio/users/resources/{subject_eth_address}` takes as path parameters the following items:
- `subject_eth_address` - The Ethereum public address of the subject of the queried resources.

The response returned is one of the following:
- `200 Ok` - Resources were found for the subject.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
	{
		"email": "user@example.com",
		"creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
		"resource_type": "insert_resource_type_here",
		"resource_id": 1,
		"ownership_claimed": true,
		"ipfs_cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi"
	},
	{
		// ...
	},
	// ...
]
```
The JSON returned is a list of JSON objects containing the following information:
- `email: String` - the email address of the user.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `resource_id: Integer` - the ID of the resource in the submitter's system.
- `ownerhsip_claimed: Boolean` - represents whether the subject of the resource has claimed it yet.
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
