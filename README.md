![dbio-logo](./readme-assets/dbio-logo.png)

# Setup Instructions

## Prerequisites

You should have an account at https://web3.storage. Get an API token from Web3.Storage and export it in your
environment as `IPFS_API_KEY="Web3.Storage API Token Here"`.

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

<br>

---

<br>

# API Documentation

## Routes

[Users](#users): Registry of the registered users in the dBio system. A new user is added when signing into the dBio client for the first time.

- [POST /dbio/users](#post-dbiousers)
- [GET /dbio/users/eth/\{eth-address\}](#get-dbiousersetheth-address)
- [GET /dbio/users/email/\{email\}](#get-dbiousersemailemail)

[Resources](#resources): FHIR resources that pertain to users. Third parties can submit resources for users, and users can view their own resources.

- [POST /dbio/resources/claimed](#post-dbioresourcesclaimed)
- [POST /dbio/resources/unclaimed](#post-dbioresourcesunclaimed)
- [GET /dbio/resources/claimed/\{subject-eth-address\}](#get-dbioresourcesclaimedsubject-eth-address)
- [GET /dbio/resources/unclaimed/\{subject-eth-address\}](#get-dbioresourcesunclaimedsubject-eth-address)
- [GET /dbio/resources/claimed/\{subject-eth-address\}/\{fhir-resource-type\}/\{fhir-resource-id\}](#get-dbioresourcesclaimedsubject-eth-addressfhir-resource-typefhir-resource-id)
- [GET /dbio/resources/unclaimed/\{subject-eth-address\}/\{fhir-resource-type\}/\{fhir-resource-id\}](#get-dbioresourcesunclaimedsubject-eth-addressfhir-resource-typefhir-resource-id)

[Read Requests](#read-requests): Access requests that are made when third parties request access to read a user's resources. Users can either approve or deny read requests.
- [POST /dbio/read_requests](#post-dbioread_requests)
- [GET /dbio/read_requests/\{requestee-eth-address\}?filter=\(open|all\)](#get-dbioread_requestsrequestee-eth-addressfilteropenall)
- [GET /dbio/read_requests/\{requestee-eth-address\}/\{requestor-eth-address\}](#get-dbioread_requestsrequestee-eth-addressrequestor-eth-address)
- [GET /dbio/read_requests/id/\{id\}](#get-dbioread_requestsidid)
- [PUT /dbio/read_requests/\{id\}?approve=\(true|false\)](#put-dbioread_requestsidapprovetruefalse)

[Write Requests](#write-requests): Access requests that are made when third parties request access to write to a user's resources. Users can either approve or deny write requests.
- [POST /dbio/write_requests](#post-dbiowrite_requests)
- [GET /dbio/write_requests/\{requestee-eth-address\}?filter=\(open|all\)](#get-dbiowrite_requestsrequestee-eth-addressfilteropenall)
- [GET /dbio/write_requests/\{requestee-eth-address\}/\{requestor-eth-address\}](#get-dbiowrite_requestsrequestee-eth-addressrequestor-eth-address)
- [GET /dbio/write_requests/id/\{id\}](#get-dbiowrite_requestsidid)
- [PUT /dbio/write_requests/\{id\}?approve=\(true|false\)](#put-dbiowrite_requestsidapprovetruefalse)

<br>

---

<br>

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

In the case of `200 Ok`, the created user object (same format as the posted JSON payload) is returned.

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

<br>

---

<br>

### Resources

#### `POST /dbio/resources/claimed`

The post request to `/dbio/resources/claimed` requires a JSON payload that represents a user to be sent in the body.
```json
{
    "email": "user@example.com",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "fhir_resource_type": "insert_resource_type_here",
    "fhir_resource_id": "insert_fhir_resource_id_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "ciphertext": "insert_ciphertext_here"
}
```
The parameters in the JSON payload are:
- `email: String` - the email address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the entity that submitted the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `fhir_resource_id: String` - the FHIR resource ID of the resource in the submitter's system.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Needed for decryption of the ciphertext.
- `ciphertext: String` - the ciphertext generated after encrypting the resource.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The resource object was created.
- `403 Forbidden` - The creator specified by the creator's Ethereum public address does not have permission to write resources for the subject. Additionally, users are unable to write to their own records.
- `404 Not Found` - No user with the specified email exists.
- `500 Internal Server Error` - A resource with the same `creator_eth_address` and `resource_id` pair already exists.

In the case of `200 Ok`, JSON is returned:
```json
{
    "fhir_resource_id": "insert_fhir_resource_id_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "fhir_resource_type": "insert_resource_type_here",
    "ipfs_cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
    "timestamp": "2022-04-16T16:22:20.949607Z"
}
```
The JSON returned represents the created resource and contains the following information:
- `fhir_resource_id: String` - the ID of the resource in the submitter's system.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `subject_eth_address: String` - the Ethereum public address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `POST /dbio/resources/unclaimed`

The post request to `/dbio/resources/unclaimed` requires a JSON payload that represents a user to be sent in the body.
```json
{
    "email": "user@example.com",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "fhir_resource_type": "insert_resource_type_here",
    "fhir_resource_id": "insert_fhir_resource_id_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "ciphertext": "insert_ciphertext_here"
}
```
The parameters in the JSON payload are:
- `email: String` - the email address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the entity that submitted the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `fhir_resource_id: String` - the FHIR resource ID of the resource in the submitter's system.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Needed for decryption of the ciphertext.
- `ciphertext: String` - the ciphertext generated after encrypting the resource.

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The resource object was created and placed in escrow.
- `403 Forbidden` - The creator specified by the creator's Ethereum public address does not have permission to write resources for the subject. Additionally, users are unable to write to their own records.
- `404 Not Found` - No user with the specified email exists.
- `500 Internal Server Error` - A resource with the same `creator_eth_address` and `resource_id` pair already exists in escrow.

In the case of `200 Ok`, JSON is returned:
```json
{
    "fhir_resource_id": "insert_fhir_resource_id_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "fhir_resource_type": "insert_resource_type_here",
    "ciphertext": "insert_ciphertext_here",
    "timestamp": "2022-04-16T16:22:20.949607Z"
}
```
The JSON returned represents the created escrowed resource and contains the following information:
- `fhir_resource_id: String` - the ID of the resource in the submitter's system.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `subject_eth_address: String` - the Ethereum public address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `ciphertext: String` - the ciphertext generated after encrypting the resource.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `GET /dbio/resources/claimed/{subject-eth-address}`

The get request to `/dbio/users/resources/claimed/{subject-eth-address}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resources.

Additionally, it takes as input a JSON payload.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address`: The Ethereum public address of the entity make the request to this route.

The response returned is one of the following:
- `200 Ok` - Resources were found for the subject.
- `403 Forbidden` - The requestor does not have access to the requested data.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
    {
        "fhir_resource_id": "insert_fhir_resource_id_here",
        "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
        "subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
        "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
        "fhir_resource_type": "insert_resource_type_here",
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
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `subject_eth_address: String` - the Ethereum public address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `GET /dbio/resources/unclaimed/{subject-eth-address}`

The get request to `/dbio/users/resources/unclaimed/{subject-eth-address}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resources.

Additionally, it takes as input a JSON payload.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address`: The Ethereum public address of the entity make the request to this route.

The response returned is one of the following:
- `200 Ok` - Resources were found for the subject.
- `403 Forbidden` - The requestor does not have access to the requested data.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
    {
        "fhir_resource_id": "insert_fhir_resource_id_here",
        "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
        "subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
        "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
        "fhir_resource_type": "insert_resource_type_here",
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
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `subject_eth_address: String` - the Ethereum public address of the subject of the resource.
- `creator_eth_address: String` - the Ethereum public address of the creator of the resource.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `ciphertext: String` - the ciphertext of the unclaimed resource.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `GET /dbio/resources/claimed/{subject-eth-address}/{fhir-resource-type}/{fhir-resource-id}`

The get request to `/dbio/users/resources/claimed/{subject-eth-address}/{fhir-resource-type}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `fhir-resource-type` - The type of the resource (correlates with a FHIR resource type).
- `fhir-resource-id` - The FHIR resource ID of the resource for which to retrieve ciphertext for.

Additionally, it takes as input a JSON payload.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address`: The Ethereum public address of the entity make the request to this route.

The response returned is one of the following:
- `200 Ok` - A resource was found for the subject.
- `403 Forbidden` - The requestor does not have access to the requested data.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
{
    "cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
    "ciphertext": "insert_ciphertext_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "fhir_resource_type": "insert_resource_type_here",
    "fhir_resource_id": "insert_fhir_resource_id_here"
}
```
The JSON returned is a JSON object containing the following information:
- `ipfs_cid: String` - the content ID of the resource in IPFS storage.
- `ciphertext: String` - the ciphertext of the FHIR resource.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `fhir_resource_id: String` - the ID of the resource in the submitter's system.

#### `GET /dbio/resources/unclaimed/{subject-eth-address}/{fhir-resource-type}/{fhir-resource-id}`

The get request to `/dbio/users/resources/unclaimed/{subject-eth-address}/{fhir-resource-type}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `fhir-resource-type` - The type of the resource (correlates with a FHIR resource type).
- `fhir-resource-id` - The FHIR resource ID of the resource for which to retrieve ciphertext for.

Additionally, it takes as input a JSON payload.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address`: The Ethereum public address of the entity make the request to this route.

The response returned is one of the following:
- `200 Ok` - A resource was found for the subject.
- `403 Forbidden` - The requestor does not have access to the requested data.
- `404 Not Found` - No resources were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
{
    "ciphertext": "insert_ciphertext_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "fhir_resource_type": "insert_resource_type_here",
    "fhir_resource_id": "insert_fhir_resource_id_here"
}
```
The JSON returned is a JSON object containing the following information:
- `ciphertext: String` - the ciphertext of the FHIR resource.
- `ironcore_document_id: String` - the ID of the related document generated by the IronCore SDK. Used to decrypt the ciphertext.
- `fhir_resource_type: String` - the type of the resource (should correlate with a FHIR resource type).
- `fhir_resource_id: String` - the ID of the resource in the submitter's system.

<br>

---

<br>

### Read Requests

#### `POST /dbio/read_requests`

The post request to `/dbio/read_requests` requires a JSON payload that represents a user to be sent in the body.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.

Upon submitting a request with well-formatted JSON, the requestor should be presented with one of the following responses:
- `200 Ok` - The access request object was created.
- `500 Internal Server Error` - This access request already exists in the system.

In the case of `200 Ok`, JSON is returned:
```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object that represents the created read request, containing the following information:
- `id: Integer` - the ID of the read request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.
- `request_approved: Boolean` - Represents whether the read request has been approved.
- `request_open: Boolean` - Represents whether the read request is still open.
- `created_time: DateTime<Utc>` - Time when the read request was submitted.
- `last_updated_time: DateTime<Utc>` - Time when the read request was updated (approved or denied).

#### `GET /dbio/read_requests/{requestee-eth-address}?filter=(open|all)`

The get request to `/dbio/read_requests/{requestee-eth-address}?filter=(open|all)` takes as path parameters the following items:
- `requestee-eth-address` - The Ethereum public address of the entity receiving the access request.

Additionally, it takes a query parameter:
- `filter` - Options are `open` or `all`. Based on the query parameter, the route returns either all read requests that a user has received, or just their open read requests.

The response returned is one of the following:
- `200 Ok` - Read requests that match the filter were found for the subject.
- `404 Not Found` - No read requests were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
    {
        "id": 10,
        "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
        "requestor_details": "Web3 Hospital of Decentralized Healthcare",
        "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
        "request_approved": false,
        "request_open": true,
        "created_time" "2022-04-30 20:57:51.733801+00",
        "last_updated_time" "2022-04-30 20:57:51.733801+00"
    },
    {
        // ...
    },
    // ...
]
```
The JSON returned is a list of JSON objects containing the following information:
- `id: Integer` - the ID of the read request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.
- `request_approved: Boolean` - Represents whether the read request has been approved.
- `request_open: Boolean` - Represents whether the read request is still open.

#### `GET /dbio/read_requests/{requestee-eth-address}/{requestor-eth-address}`

The get request to `/dbio/read_requests/{requestee-eth-address}/{requestor-eth-address}` takes as path parameters the following items:
- `requestor_eth_address` - The Ethereum public address of the entity making the read request.
- `requestee_eth_address` - The Ethereum public address of the entity receiving the read request.

The response returned is one of the following:
- `200 Ok` - The read request with the associated Ethereum addresses was found.
- `404 Not Found` - No read requests were found.

In the case of `200 Ok`, the body of the response contains JSON.

```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object containing the following information:
- `id: Integer` - the ID of the read request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.
- `request_approved: Boolean` - Represents whether the read request has been approved.
- `request_open: Boolean` - Represents whether the read request is still open.

#### `GET /dbio/read_requests/id/{id}`

The get request to `/dbio/read_requests/id/{id}` takes as path parameters the following items:
- `id` - The ID of the read request to check the status of.

The response returned is one of the following:
- `200 Ok` - The read request was successfully approved or denied.
- `404 Not Found` - No read request with the given ID was found.

In the case of `200 Ok`, the body of the response contains JSON.

```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object containing the following information:
- `id: Integer` - the ID of the read request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.
- `request_approved: Boolean` - Represents whether the read request has been approved.
- `request_open: Boolean` - Represents whether the read request is still open.

#### `PUT /dbio/read_requests/{id}?approve=(true|false)`

The put request to `/dbio/read_requests/{id}?approve=(true|false)` takes as path parameters the following items:
- `id` - The ID of the read request being responded to.

Additionally, it takes a query parameter:
- `approve` - Options are `true` or `false`. Based on the query parameter, the read request is either approved or denied.

The response returned is one of the following:
- `200 Ok` - The read request was successfully approved or denied.
- `404 Not Found` - No read request with the given ID was found.

In the case of `200 Ok`, JSON is returned:
```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object that represents the updated read request, containing the following information:
- `id: Integer` - the ID of the read request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the read request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the read request.
- `request_approved: Boolean` - Represents whether the read request has been approved.
- `request_open: Boolean` - Represents whether the read request is still open.

<br>

---

<br>

### Write Requests

#### `POST /dbio/write_requests`

The post request to `/dbio/write_requests` requires a JSON payload that represents a user to be sent in the body.
```json
{
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240"
}
```
The parameters in the JSON payload are:
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.

Upon submitting a request with well-formatted JSON, the requestor should be presented with one of the following responses:
- `200 Ok` - The write request object was created.
- `500 Internal Server Error` - This write request already exists in the system.

In the case of `200 Ok`, JSON is returned:
```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object that represents the created write request, containing the following information:
- `id: Integer` - the ID of the write request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.
- `request_approved: Boolean` - Represents whether the write request has been approved.
- `request_open: Boolean` - Represents whether the write request is still open.
- `created_time: DateTime<Utc>` - Time when the write request was submitted.
- `last_updated_time: DateTime<Utc>` - Time when the write request was last updated (approved or denied).

#### `GET /dbio/write_requests/{requestee-eth-address}?filter=(open|all)`

The get request to `/dbio/write_requests/{requestee-eth-address}?filter=(open|all)` takes as path parameters the following items:
- `requestee-eth-address` - The Ethereum public address of the entity receiving the write request.

Additionally, it takes a query parameter:
- `filter` - Options are `open` or `all`. Based on the query parameter, the route returns either all write requests that a user has received, or just their open write requests.

The response returned is one of the following:
- `200 Ok` - Write requests that match the filter were found for the subject.
- `404 Not Found` - No write requests were found for the subject.

In the case of `200 Ok`, the body of the response contains JSON.
```json
[
    {
        "id": 10,
        "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
        "requestor_details": "Web3 Hospital of Decentralized Healthcare",
        "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
        "request_approved": false,
        "request_open": true,
        "created_time" "2022-04-30 20:57:51.733801+00",
        "last_updated_time" "2022-04-30 20:57:51.733801+00"
    },
    {
        // ...
    },
    // ...
]

```
The JSON returned is a list of JSON objects containing the following information:
- `id: Integer` - the ID of the write request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.
- `request_approved: Boolean` - Represents whether the write request has been approved.
- `request_open: Boolean` - Represents whether the write request is still open.

#### `GET /dbio/write_requests/{requestee-eth-address}/{requestor-eth-address}`

The get request to `/dbio/read_requests/{requestee-eth-address}/{requestor-eth-address}` takes as path parameters the following items:
- `requestor_eth_address` - The Ethereum public address of the entity making the write request.
- `requestee_eth_address` - The Ethereum public address of the entity receiving the write request.

The response returned is one of the following:
- `200 Ok` - The write request with the associated Ethereum addresses was found.
- `404 Not Found` - No write requests were found.

In the case of `200 Ok`, the body of the response contains JSON.

```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object containing the following information:
- `id: Integer` - the ID of the write request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.
- `request_approved: Boolean` - Represents whether the write request has been approved.
- `request_open: Boolean` - Represents whether the write request is still open.

#### `GET /dbio/write_requests/id/{id}`

The get request to `/dbio/write_requests/id/{id}` takes as path parameters the following items:
- `id` - The ID of the write request to check the status of.

The response returned is one of the following:
- `200 Ok` - The write request was successfully approved or denied.
- `404 Not Found` - No write request with the given ID was found.

In the case of `200 Ok`, the body of the response contains JSON.

```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object containing the following information:
- `id: Integer` - the ID of the write request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.
- `request_approved: Boolean` - Represents whether the write request has been approved.
- `request_open: Boolean` - Represents whether the write request is still open.

#### `PUT /dbio/write_requests/{id}?approve=(true|false)`

The put request to `/dbio/write_requests/{id}?approve=(true|false)` takes as path parameters the following items:
- `id` - The ID of the write request being responded to.

Additionally, it takes a query parameter:
- `approve` - Options are `true` or `false`. Based on the query parameter, the write request is either approved or denied.

The response returned is one of the following:
- `200 Ok` - The write request was successfully approved or denied.
- `404 Not Found` - No write request with the given ID was found.

In the case of `200 Ok`, JSON is returned:
```json
{
    "id": 10,
    "requestor_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "requestor_details": "Web3 Hospital of Decentralized Healthcare",
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "request_approved": false,
    "request_open": true,
    "created_time" "2022-04-30 20:57:51.733801+00",
    "last_updated_time" "2022-04-30 20:57:51.733801+00"
}
```
The JSON returned is an object that represents the updated write request, containing the following information:
- `id: Integer` - the ID of the write request.
- `requestor_eth_address: String` - The Ethereum public address of the entity making the write request.
- `requestor_details: String` - Information that identifies the requestor as a real world entity (e.g. name of an organization).
- `requestee_eth_address: String` - The Ethereum public address of the entity receiving the write request.
- `request_approved: Boolean` - Represents whether the write request has been approved.
- `request_open: Boolean` - Represents whether the write request is still open.
