-- Create the tags table
CREATE TABLE tags (
    name VARCHAR PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger to update the updated_at timestamp for tags
CREATE TRIGGER update_tags_updated_at
BEFORE UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Create the item_tags table (junction table)
CREATE TABLE item_tags (
    item_id UUID NOT NULL,
    tag VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Set primary key as the combination of item_id and tag
    PRIMARY KEY (item_id, tag),
    
    -- Add foreign key constraints
    CONSTRAINT fk_item
        FOREIGN KEY(item_id)
        REFERENCES items(id)
        ON DELETE CASCADE,
    
    CONSTRAINT fk_tag
        FOREIGN KEY(tag)
        REFERENCES tags(name)
        ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX idx_item_tags_item_id ON item_tags(item_id);
CREATE INDEX idx_item_tags_tag ON item_tags(tag);
