# Setup Instructions

{Insert setup instructions here}

# API Documentation

## Routes

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
