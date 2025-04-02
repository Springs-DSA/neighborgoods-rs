-- Create the item_cert_requirements table (join table between items and certifications)
CREATE TABLE item_cert_requirements (
    item_id UUID NOT NULL,
    cert_id UUID NOT NULL,
    purposes transfer_purpose[] NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Add primary key constraint on both columns (composite key)
    PRIMARY KEY (item_id, cert_id),
    
    -- Add foreign key constraints
    CONSTRAINT fk_item
        FOREIGN KEY(item_id)
        REFERENCES items(id)
        ON DELETE CASCADE,
    
    CONSTRAINT fk_cert
        FOREIGN KEY(cert_id)
        REFERENCES certifications(id)
        ON DELETE CASCADE
);

-- Create a trigger to update the updated_at timestamp
CREATE TRIGGER update_item_cert_requirements_updated_at
BEFORE UPDATE ON item_cert_requirements
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
