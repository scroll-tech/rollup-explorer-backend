CREATE TYPE block_status AS ENUM ('uncommitted', 'committed', 'verified');

CREATE TABLE l2_blocks (
    id BIGINT PRIMARY KEY,
    status block_status NOT NULL DEFAULT 'uncommitted',
    header_hash VARCHAR(66) NOT NULL,
    l1_tx_hash VARCHAR(66) DEFAULT NULL,
    tx_num BIGINT NOT NULL DEFAULT 0,
    timestamp NUMERIC(78) CHECK (timestamp >= 0 AND timestamp < 2^256) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX uni_idx_id_on_l2_blocks on l2_blocks (id);
CREATE INDEX idx_status_on_l2_blocks on l2_blocks (status);
CREATE INDEX idx_created_at_on_l2_blocks on l2_blocks (created_at);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_timestamp BEFORE UPDATE
ON l2_blocks FOR EACH ROW EXECUTE PROCEDURE
update_timestamp();
