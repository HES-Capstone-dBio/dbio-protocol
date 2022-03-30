CREATE TABLE IF NOT EXISTS users (
  eth_public_address CHAR(42) PRIMARY KEY,
  email VARCHAR(320) NOT NULL
);

CREATE INDEX IF NOT EXISTS user_email ON users (email);

CREATE TABLE IF NOT EXISTS resources (
  fhir_resource_id BIGINT NOT NULL,
  subject_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  creator_eth_address CHAR(42) REFERENCES users(eth_public_address) NOT NULL,
  resource_type VARCHAR(40) NOT NULL,
  ownership_claimed BOOL NOT NULL DEFAULT false,
  ipfs_cid VARCHAR(50) NOT NULL,
  PRIMARY KEY (subject_eth_address, fhir_resource_id)
);

CREATE INDEX IF NOT EXISTS resources_subject_address ON resources (subject_eth_address);
