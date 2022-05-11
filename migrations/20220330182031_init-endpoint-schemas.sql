SET timezone = 'America/New_York';

CREATE TABLE IF NOT EXISTS users (
  eth_public_address CHAR(42) PRIMARY KEY,
  email VARCHAR(320) NOT NULL
);

CREATE INDEX IF NOT EXISTS user_email ON users (email);

CREATE TABLE IF NOT EXISTS resources (
  fhir_resource_id VARCHAR(64) NOT NULL,
  ironcore_document_id CHAR(32) NOT NULL,
  subject_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_details VARCHAR NOT NULL,
  fhir_resource_type VARCHAR(40) NOT NULL,
  ipfs_cid VARCHAR NOT NULL,
  eth_nft_voucher VARCHAR NOT NULL,
  nft_minted BOOL NOT NULL DEFAULT false,
  timestamp TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (creator_eth_address, fhir_resource_id)
);

CREATE TABLE IF NOT EXISTS resource_escrow (
  fhir_resource_id VARCHAR(64) NOT NULL,
  ironcore_document_id CHAR(32) NOT NULL,
  subject_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_details VARCHAR NOT NULL,
  fhir_resource_type VARCHAR(40) NOT NULL,
  ciphertext VARCHAR NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (creator_eth_address, fhir_resource_id)
);

CREATE INDEX IF NOT EXISTS resources_subject_address ON resources (subject_eth_address);

CREATE TABLE IF NOT EXISTS read_requests (
  id BIGSERIAL PRIMARY KEY,
  requestor_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  requestor_details VARCHAR(128) NOT NULL,
  requestee_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  request_approved BOOL NOT NULL DEFAULT false,
  request_open BOOL NOT NULL DEFAULT true,
  created_time TIMESTAMPTZ NOT NULL,
  last_updated_time TIMESTAMPTZ NOT NULL,
  CONSTRAINT read_request_pair UNIQUE(requestee_eth_address, requestor_eth_address)
);

CREATE INDEX IF NOT EXISTS read_requests_requestee ON read_requests (requestee_eth_address);

CREATE TABLE IF NOT EXISTS write_requests (
  id BIGSERIAL PRIMARY KEY,
  requestor_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  requestor_details VARCHAR(128) NOT NULL,
  requestee_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  request_approved BOOL NOT NULL DEFAULT false,
  request_open BOOL NOT NULL DEFAULT true,
  created_time TIMESTAMPTZ NOT NULL,
  last_updated_time TIMESTAMPTZ NOT NULL,
  CONSTRAINT write_request_pair UNIQUE(requestee_eth_address, requestor_eth_address)
);

CREATE INDEX IF NOT EXISTS write_requests_requestee ON write_requests (requestee_eth_address);
