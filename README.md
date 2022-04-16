![dbio-logo](./readme-assets/dbio-logo.png)

# Setup Instructions

## Setup for Demo, Testing, and Experimentation

If you simply wish to run the `dbio-protocol` server locally for API testing,
the `docker-compose.yml` file in this repository composes together the most recent
protocol server code, the most recent `dbio-client` UI image, and a Postgres database.

You can run `docker compose up` or `docker compose up --detach` from this
directory to spin up the entire dBio application as a set of connected containers.
The whole setup should take about 5 minutes to build and run on a modern Macbook.
Once running, the protocol server will bind to `localhost:8080` and the UI at `localhost:3000`.

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

**Important:**
When developing actual code in this repository and any `sqlx` queries or database schemas are changed,
you must run `cargo sqlx prepare`. This command regenerates the `sqlx-data.json` file, which should then
be checked in and committed, as it is used when compiling offline and building the project's Docker image.
The build will fail without completing this step.

# API Documentation

## Routes

[Users](#users): Registry of the registered users in the dBio system. A new user is added when signing into the dBio client for the first time.

- [POST /dbio/users](#post-dbiousers)
- [GET /dbio/users/eth/\{eth-address\}](#get-dbiousersetheth-address)
- [GET /dbio/users/email/{email}](#get-dbiousersemailemail)

[Resources](#resources): FHIR resources that pertain to users. Third parties can submit resources for users, and users can view their own resources.

- [POST /dbio/resources](#post-dbioresources)
- [GET /dbio/resources/\{subject-eth-address\}](#get-dbioresourcessubject-eth-address)
- [GET /dbio/resources/\{subject-eth-address\}/\{resource-id\}](#get-dbioresourcessubject-eth-addressresource-id)
- [PUT /dbio/resources/claim/\{subject-eth-address\}/\{resource-id\}](#put-dbioresourcesclaimsubject-eth-addressresource-id)

[Access Requests](#access-requests): Access requests that are made when third parties request access to a user's resources. Users can either approve or deny access requests.
- [POST /dbio/access_requests](#post-dbioaccess_requests)
- [GET /dbio/access_requests/\{requestee-eth-address\}?filter=\(open|all\)](#get-dbioaccess_requestsrequestee-eth-addressfilteropenall)
- [PUT /dbio/access_requests/{id}?approve=(true|false)](#put-dbioaccess_requestsidapprovetruefalse)

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
- `500 Internal Server Error` - The user object already exists.

#### `GET /dbio/users/eth/{eth-address}`
The get request to `/dbio/users/eth/{eth-address}` takes as path parameters the following items:
- `eth-address` - The Ethereum public address of the user being queried.

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
	"resource_title": "insert_resource_title_here",
	"fhir_resource_id": "insert_fhir_resource_id_here",
	"ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
	"ciphertext": "insert_ciphertext_here"
}
```
The parameters in the JSON payload are:
- `email: String` - the email address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the entity submitting the resource.
- `resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `resource_title: String` - the title of the resource as designated by the submitting party.
- `fhir_resource_id: String` - the FHIR resource ID of the resource in the submitter's system.
- `ironcore_document_id` - the ID of the related document generated by the IronCore SDK. Needed for decryption of the ciphertext.
- `ciphertext: String` - the ciphertext generated after encrypting the resource.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The resource object was created.
- `404 Not Found` - No user with the specified email exists.
- `500 Internal Server Error` - A resource with the same `creator_eth_address` and `resource_id` pair already exists.

#### `GET /dbio/resources/{subject-eth-address}`
The get request to `/dbio/users/resources/{subject-eth-address}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resources.

The response returned is one of the following:
- `200 Ok` - Resources were found for the subject.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
	{
		"fhir_resource_id": "insert_fhir_resource_id_here",
		"ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
		"subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
		"creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
		"resource_type": "insert_resource_type_here",
		"resource_title": "insert_resource_title_here",
		"ownership_claimed": true,
		"ipfs_cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
		"timestamp": "2022-04-16T16:22:20.949607Z"
	},
	{
		// ...
	},
	// ...
]
```
The JSON returned is a list of JSON objects containing the following information:
- `fhir_resource_id: String` - the ID of the resource in the submitter's system.
- `ironcore_document_id` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `subject_eth_address: String` - the Ethereum public address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `resource_title: String` - the title of the resource.
- `ownerhsip_claimed: Boolean` - represents whether the subject of the resource has claimed it yet.
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `GET /dbio/resources/{subject-eth-address}/{resource-id}`
The get request to `/dbio/users/resources/{subject-eth-address}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `resource-id` - The ID of the resource for which to retrieve ciphertext for.

The response returned is one of the following:
- `200 Ok` - A resource was found for the subject.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
{
	"cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
	"ciphertext": "insert_ciphertext_here"
}
```
The JSON returned is a JSON object containing the following information:
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
- `ciphertext` - the ciphertext of the FHIR resource.

#### `PUT /dbio/resources/claim/{subject-eth-address}/{resource-id}`
The put request to `/dbio/resources/claim/{subject-eth-address}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `resource-id` - The ID of the resource being claimed.

The response returned is one of the following:
- `200 Ok` - A resource was found for the subject and was successfully claimed.
- `404 Not Found` - No resource matching that resource ID was found for the subject.

### Access Requests

#### `POST /dbio/access_requests`
The post request to `/dbio/access_requests` requires a JSON payload that represents a user to be sent in the body.
```json
{
	"requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
	"requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address : String` - The Ethereum public address of the entity making the access request.
- `requestee_eth_address : String` - The Ethereum public address of the entity receiving the access request.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The access request object was created.
- `500 Internal Server Error` - This access request already exists in the system.

#### `GET /dbio/access_requests/{requestee-eth-address}?filter=(open|all)`
The get request to `/dbio/access_requests/{requestee-eth-address}?filter=(open|all)` takes as path parameters the following items:
- `requestee-eth-address` - The Ethereum public address of the entity receiving the access request.

Additionally, it takes a query parameter:
- `filter` - Options are `open` or `all`. Based on the query parameter, the route returns either all access requests that a user has received, or just their open access requests.

The response returned is one of the following:
- `200 Ok` - Access requests that match the filter were found for the subject.
- `404 Not Found` - No access requests were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
	{
		"id": 10,
		"requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
		"requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
		"request_approved": false,
		"request_open": true,
	},
	{
		// ...
	},
	// ...
]

```
The JSON returned is a list of JSON objects containing the following information:
- `id: Integer` - the ID of the access request.
- `requestor_eth_address : String` - The Ethereum public address of the entity making the access request.
- `requestee_eth_address : String` - The Ethereum public address of the entity receiving the access request.
- `request_approved : Boolean` - Represents whether the access request has been approved.
- `request_open : Boolean` - Represents whether the access request is still open.

#### `PUT /dbio/access_requests/{id}?approve=(true|false)`

The put request to `/dbio/access_requests/{id}?approve=(true|false)` takes as path parameters the following items:
- `id` - The ID of the access request being responded to.

Additionally, it takes a query parameter:
- `approve` - Options are `true` or `false`. Based on the query parameter, the access request is either approved or denied.

The response returned is one of the following:
- `200 Ok` - The access request was successfully approved or denied.
- `404 Not Found` - No access request with the given ID was found.
