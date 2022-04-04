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
- `eth_public_address: String` - the Ethereum public address of the user
- `email: String` - the email address of the user

Upon submitting a request with well-formatted JSON, the requester should be presented with one of the following responses:
- `200 Ok` - The user object was created.
- `409 Conflict` - The user object already exists.
