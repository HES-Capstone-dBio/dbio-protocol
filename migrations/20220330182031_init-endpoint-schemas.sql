CREATE TABLE IF NOT EXISTS users (
  eth_public_address CHAR(42) PRIMARY KEY,
  email VARCHAR(320) NOT NULL
);

CREATE INDEX IF NOT EXISTS user_email ON users (email);

CREATE TABLE IF NOT EXISTS resource_store (
  cid VARCHAR(50) PRIMARY KEY,
  ciphertext VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS resources (
  fhir_resource_id BIGINT NOT NULL,
  subject_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  resource_type VARCHAR(40) NOT NULL,
  ownership_claimed BOOL NOT NULL DEFAULT false,
  ipfs_cid VARCHAR(50) REFERENCES resource_store(cid) NOT NULL,
  PRIMARY KEY (creator_eth_address, fhir_resource_id)
);

CREATE INDEX IF NOT EXISTS resources_subject_address ON resources (subject_eth_address);

CREATE TABLE IF NOT EXISTS access_requests (
  id BIGSERIAL PRIMARY KEY,
  requestor_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  requestee_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  request_approved BOOL NOT NULL DEFAULT false,
  request_open BOOL NOT NULL DEFAULT true
);

CREATE INDEX IF NOT EXISTS access_requests_requestee ON access_requests (requestee_eth_address);

