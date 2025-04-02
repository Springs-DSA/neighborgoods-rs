-- Create ENUMs for purpose and status
CREATE TYPE transfer_purpose AS ENUM ('USE', 'MAINTAIN', 'RESTOCK', 'CONSUME', 'CONTRIBUTE', 'DELIST');
CREATE TYPE transfer_status AS ENUM ('RESERVED', 'COMPLETED', 'CANCELED');

-- Create the item_transfers table
CREATE TABLE item_transfers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    item_id UUID NOT NULL,
    steward_id UUID NOT NULL,
    prev_steward_id UUID,  -- Nullable to preserve transfer history if a user leaves
    purpose transfer_purpose NOT NULL,
    lat DECIMAL(10, 6),
    lng DECIMAL(10, 6),
    status transfer_status NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Add foreign key constraints
    CONSTRAINT fk_item
        FOREIGN KEY(item_id)
        REFERENCES items(id)
        ON DELETE CASCADE,
    
    CONSTRAINT fk_steward
        FOREIGN KEY(steward_id)
        REFERENCES users(id)
        ON DELETE CASCADE,
        
    CONSTRAINT fk_prev_steward
        FOREIGN KEY(prev_steward_id)
        REFERENCES users(id)
        ON DELETE SET NULL
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_item_transfers_updated_at
BEFORE UPDATE ON item_transfers
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
