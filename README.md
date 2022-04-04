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
