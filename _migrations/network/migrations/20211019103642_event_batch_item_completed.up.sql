CREATE TABLE IF NOT EXISTS sub_event_batch_item_completed
(
    id              SERIAL PRIMARY KEY,
    block_hash      VARCHAR(66) NOT NULL,
    extrinsic_index integer,
    nesting_index   text,
    event_index     integer NOT NULL,
    created_at      TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT sub_event_batch_item_completed_u_event
        UNIQUE (block_hash, event_index),
    CONSTRAINT sub_event_batch_item_completed_fk_block
        FOREIGN KEY (block_hash)
            REFERENCES sub_block (hash)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS sub_event_batch_item_completed_idx_block_hash
    ON sub_event_batch_item_completed (block_hash);