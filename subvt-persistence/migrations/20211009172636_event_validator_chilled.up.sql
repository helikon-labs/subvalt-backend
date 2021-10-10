CREATE TABLE IF NOT EXISTS event_validator_chilled
(
    block_hash           VARCHAR(66)                 NOT NULL,
    extrinsic_index      integer,
    validator_account_id VARCHAR(66)                 NOT NULL,
    last_updated         TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT event_validator_chilled_u_block_hash_account_id
        UNIQUE (block_hash, validator_account_id),
    CONSTRAINT event_validator_chilled_fk_block
        FOREIGN KEY (block_hash)
            REFERENCES block (hash)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT event_validator_chilled_fk_account
        FOREIGN KEY (validator_account_id)
            REFERENCES account (id)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);