![dbio-logo](./readme-assets/dbio-logo.png)

# About

The dBio protocol server is a part of the overall [dBio application / protocol](https://github.com/HES-Capstone-dBio).
It contains the server logic to store and retrieve metadata, as well as the logic to store encrypted texts to
IPFS and sign vouchers for minting NFTs. Both the [dBio Client](https://github.com/HES-Capstone-dBio/dbio-client)
and the [dbio FHIR Proxy](https://github.com/HES-Capstone-dBio/dbio-fhir-proxy) depend on the
protocol server via API calls. The dBio protocol server persists data with a Postgres database.

<br>

---

<br>


# Setup Instructions

## Prerequisites

### Web3.Storage

You should have an account at https://web3.storage. Get an API token from Web3.Storage. This will
be exported it in your environment as `IPFS_API_KEY="Web3.Storage API Token Here"`. Web3.Storage
is used as an API to submit data to IPFS.

### Rust

You should have an installation of `cargo` and `rustup`. This can be done via your package manager
of choice, or it can be done by following the instructions at https://rustup.rs/.
Additionally, you need to install Rust, which can be done via the command `rustup toolchain install stable`.

### `SQLX`

`SQLX` is a library that optionally validates code against a running SQL database.
More information about `SQLX` and its installation can be found at https://github.com/launchbadge/sqlx#install.
Additionally, it is highly recommended to install the `sqlx-cli`, for which installation
instructions can be found at https://crates.io/crates/sqlx-cli. Installation of the `sqlx-cli` is
necessary to run some commands for migrations if either SQL schemas or SQL queries in the codebase are changed.

### Docker

Docker is used to containerize the application. Installation instructions can be found
at https://www.docker.com/products/docker-desktop/.

## Setup

### Setup for Production and Demos

#### With Docker

If you simply wish to run the `dbio-protocol` server locally for API testing,
the `docker-compose.yml` file in this repository composes together the most recent
protocol server code, the most recent `dbio-client` UI image, the most recent `dbio-fhir-proxy`
image, and a Postgres database.
If you do not wish to run either the client or the FHIR proxy, you can comment out the
corresponding parts in `docker-compose.yml` in the `services` section.
Note: the `dbio-protocol` server depends on a running instance of the Postgres database.
Commenting out the `dbio-postgres` section of `docker-compose.yml` will result in failure
of the `dbio-protocol` server to start up.

You can run `docker compose up` or `docker compose up --detach` from this
directory to spin up the entire dBio application as a set of connected containers.
The whole setup should take about 5 minutes to build and run on a modern Macbook.
Once running, the protocol server will bind to `localhost:8080`, the DHIR proxy
to `localhost:8081`, and the UI to `localhost:3000`. At this point, you can interact
with running instances of all of the components of the dBio application.

#### Locally

If you wish to run the `dbio-protocol` server outside of a Docker container, do the following
steps:

1. Start *only* the Postgres Docker container with `docker compose -f ./docker-compose-dev.yml up`.
   This will start up the necessary Postgres database.
2. Additionally, export the environmental variables:
   - `export IPFS_API_KEY={Web3.Storage API Key here}`, using the API key from Web3.Storage.
   - `export DATABASE_URL=postgres://postgres:password@localhost/dbio-protocol-db`.
     Alternatively, you can uncomment the line with `DATABASE_URL` in the `.env` file.
3. Run the command `SQLX_OFFLINE=true cargo build --release`. This will build the application.
4. Run the command `./target/release/dbio-protocol`. This will run the application.

### Setup for Development

#### Local Build for Testing and Development

Follow these steps:

1. Start *only* the Postgres Docker container with `docker compose -f ./docker-compose-dev.yml up`.
   This will start up the necessary Postgres database.
2. Additionally, export the environmental variables:
   - `export IPFS_API_KEY={Web3.Storage API Key here}`, using the API key from Web3.Storage.
   - `export DATABASE_URL=postgres://postgres:password@localhost/dbio-protocol-db`.
     Alternatively, you can uncomment the line with `DATABASE_URL` in the `.env` file.
3. Run the command `SQLX_OFFLINE=true cargo build`. This will build the application.
   Alternatively, you can run directly with the command `SQLX_OFFLINE=true cargo run`, and
   skip step 4.
4. Run the command `./target/debug/dbio-protocol`. This will run the application (applicable
   only if you ran the `cargo build` command in step 3).

### Development Information

#### **SQLX Offline Mode & Migrations**
When developing actual code in this repository and any `sqlx` queries or database schemas are changed,
you must run `cargo sqlx prepare`. This command regenerates the `sqlx-data.json` file, which should then
be checked in and committed, as it is used when compiling offline and building the project's Docker image.
The build will fail without completing this step.

#### Compiling and Running Rust Programs
To check Rust code for errors (as well as errors in dependencies), run `cargo check`.
To build a Rust program, run `cargo build`. The generated executable is at `./target/debug/dbio-protocol`.
To build a more optimized version for production, run `cargo build --release`. The generated executable is
at `./target/release/dbio-protocol`.
To compile and run directly, run `cargo run`.
To clean up the `target` directory, run `cargo clean`.

<<<<<<< HEAD
<<<<<<< HEAD
#### Compiling and Running Rust Programs
To check Rust code for errors (as well as errors in dependencies), run `cargo check`.
To build a Rust program, run `cargo build`. The generated executable is at `./target/debug/dbio-protocol`.
To build a more optimized version for production, run `cargo build --release`. The generated executable is
at `./target/release/dbio-protocol`.
To compile and run directly, run `cargo run`.
To clean up the `target` directory, run `cargo clean`.
=======
=======
>>>>>>> 2eec800c1a7c9c61b95a96bafe658f184fd4603e
#### Testing
Included in this repository is a file called `dbio-protocol.postman_collection.json` that contains
[Postman](https://www.postman.com/) API calls that were used in the testing of this protocol server.
Installation instructions for Postman are at https://www.postman.com/downloads/.
Instructions to import the collection can be found at https://learning.postman.com/docs/getting-started/importing-and-exporting-data/#contents.
Once the collection is imported, these API calls can be run once there is a running instance of the
protocol server and the Postgres database.

#### Editing Code
Edits made to the codebase of the protocol server can affect the [dBio Client](https://github.com/HES-Capstone-dBio/dbio-client)
and the [dbio FHIR Proxy](https://github.com/HES-Capstone-dBio/dbio-fhir-proxy). This applies
especially to any edits made to database operations or routes, as the dBio Client and FHIR proxy
depend on the dBio protocol API as part of their functionality. If edits are made to the dBio
protocol server, these changes will likely affect the other parts of the dBio application, and they
should be changed accordingly.
<<<<<<< HEAD
>>>>>>> a4d65dc (testing readme, about section, edits section)
=======
>>>>>>> 2eec800c1a7c9c61b95a96bafe658f184fd4603e

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
- [GET /dbio/resources/claimed/\{subject-eth-address\}/\{reader-eth-address\}](#get-dbioresourcesclaimedsubject-eth-addressreader-eth-address)
- [GET /dbio/resources/unclaimed/\{subject-eth-address\}/\{reader-eth-address\}](#get-dbioresourcesunclaimedsubject-eth-addressreader-eth-address)
- [GET /dbio/resources/claimed/\{subject-eth-address\}/\{fhir-resource-type\}/\{fhir-resource-id\}/\{reader-eth-address\}](#get-dbioresourcesclaimedsubject-eth-addressfhir-resource-typefhir-resource-idreader-eth-address)
- [GET /dbio/resources/unclaimed/\{subject-eth-address\}/\{fhir-resource-type\}/\{fhir-resource-id\}/\{reader-eth-address\}](#get-dbioresourcesunclaimedsubject-eth-addressfhir-resource-typefhir-resource-idreader-eth-address)
- [PUT dbio/resources/claimed/mint/\{creator\_eth\_address\}/{fhir\_resource\_id\}\?minted\=\(true\|false\)](#put-dbioresourcesclaimedmintcreator_eth_addressfhir_resource_idmintedtruefalse)

[Read Requests](#read-requests): Access requests that are made when third parties request access to read a user's resources. Users can either approve or deny read requests.
- [POST /dbio/read_requests](#post-dbioread_requests)
- [GET /dbio/read_requests/\{requestee-eth-address\}?filter=\(open|all\)](#get-dbioread_requestsrequestee-eth-addressfilteropenall)
- [GET /dbio/read_requests/id/\{id\}](#get-dbioread_requestsidid)
- [PUT /dbio/read_requests/\{id\}?approve=\(true|false\)](#put-dbioread_requestsidapprovetruefalse)

[Write Requests](#write-requests): Access requests that are made when third parties request access to write to a user's resources. Users can either approve or deny write requests.
- [POST /dbio/write_requests](#post-dbiowrite_requests)
- [GET /dbio/write_requests/\{requestee-eth-address\}?filter=\(open|all\)](#get-dbiowrite_requestsrequestee-eth-addressfilteropenall)
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
    "eth_nft_voucher": "{ \"uri\": \"https://ipfs.io/ipfs/bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi\", \"signature\": \"0x793fe62fb070cfd09869b765fb3b70f5dc85572f68dcada36d6e92d362463bed38671da834cc7dc3f92fa6962afb996de90c6e84fcc8d1b3d08bc4fd7518c2b31b\" }",
    "nft_minted": false,
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
- `eth_nft_voucher`: an IPFS URI accompanied by an EIP-712 signature, as JSON string
- `nft_minted`: status of minting to be updated by client side (default is false)
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

#### `GET /dbio/resources/claimed/{subject-eth-address}/{reader-eth-address}`

The get request to `/dbio/users/resources/claimed/{subject-eth-address}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resources.
- `reader-eth-address` - The Ethereum public address of the entity made the request to get the resource.

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
        "eth_nft_voucher": "{ \"uri\": \"https://ipfs.io/ipfs/bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi\", \"signature\": \"0x793fe62fb070cfd09869b765fb3b70f5dc85572f68dcada36d6e92d362463bed38671da834cc7dc3f92fa6962afb996de90c6e84fcc8d1b3d08bc4fd7518c2b31b\" }",
        "nft_minted": false,
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
- `eth_nft_voucher: String` - serialized JSON of an Ethereum NFT voucher with
- `nft_minted: Boolean` - a boolean representing whether or not the user has
  minted an NFT for this resource yet. Defaults to false.
- `timestamp: String` - the timestamp at which the resource was created, formatted as per ISO 8601 standards.

#### `GET /dbio/resources/unclaimed/{subject-eth-address}/{reader-eth-address}`

The get request to `/dbio/users/resources/unclaimed/{subject-eth-address}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resources.
- `reader-eth-address` - The Ethereum public address of the entity made the request to get the resource.

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

#### `GET /dbio/resources/claimed/{subject-eth-address}/{fhir-resource-type}/{fhir-resource-id}/{reader-eth-address}`

The get request to `/dbio/users/resources/claimed/{subject-eth-address}/{fhir-resource-type}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `fhir-resource-type` - The type of the resource (correlates with a FHIR resource type).
- `fhir-resource-id` - The FHIR resource ID of the resource for which to retrieve ciphertext for.
- `reader-eth-address` - The Ethereum public address of the entity made the request to get the resource.

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

#### `GET /dbio/resources/unclaimed/{subject-eth-address}/{fhir-resource-type}/{fhir-resource-id}/{reader-eth-address}`

The get request to `/dbio/users/resources/unclaimed/{subject-eth-address}/{fhir-resource-type}/{resource-id}` takes as path parameters the following items:
- `subject-eth-address` - The Ethereum public address of the subject of the queried resource.
- `fhir-resource-type` - The type of the resource (correlates with a FHIR resource type).
- `fhir-resource-id` - The FHIR resource ID of the resource for which to retrieve ciphertext for.
- `reader-eth-address` - The Ethereum public address of the entity made the request to get the resource.

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

#### `PUT dbio/resources/claimed/mint/{creator_eth_address}/{fhir_resource_id}?minted=(true|false)`

Updates the `nft_minted` status to the value of boolean query parameter `minted`.
Requires path parameters `creator_eth_address` and `fhir_resource_id`, both of which are strings
and refer to the resource in question by the primary key on the `resources` table.

Return value is a `Resource` payload with the `nft_minted` field updated to the specified
value.

```
{
    "fhir_resource_id": "insert_fhir_resource_id_here",
    "ironcore_document_id": "2b544876c9ec9fa56c800c3a2235fdbd",
    "subject_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240",
    "creator_eth_address": "0xA6f03f794286C60392450438406b3Ebf2878F584",
    "fhir_resource_type": "insert_resource_type_here",
    "ipfs_cid": "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
    "eth_nft_voucher": "{ \"uri\": \"https://ipfs.io/ipfs/bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi\", \"signature\": \"0x793fe62fb070cfd09869b765fb3b70f5dc85572f68dcada36d6e92d362463bed38671da834cc7dc3f92fa6962afb996de90c6e84fcc8d1b3d08bc4fd7518c2b31b\" }",
    "nft_minted": true,
    "timestamp": "2022-04-16T16:22:20.949607Z"
}
```

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
    "requestee_eth_address": "0xE2b01f344355A01331470417711b1Dca1982A240", "request_approved": false, "request_open": true,
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

