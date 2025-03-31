CREATE TABLE node_settings (
    id SERIAL PRIMARY KEY,
    entity VARCHAR NOT NULL,
    attribute VARCHAR NOT NULL,
    value TEXT NOT NULL,
    data_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (entity, attribute)
);

-- Initial default settings
INSERT INTO node_settings (entity, attribute, value, data_type) VALUES
    ('node', 'name', 'NeighborGoods Local Node', 'string'),
    ('node', 'id', '1', 'integer'),
    ('node', 'description', 'A local instance of the NeighborGoods federated library of things', 'string'),
    ('item', 'budget_per_person', '5', 'integer'),
    ('collective_action', 'min_votes', '3', 'integer'),
    ('collective_action', 'expiry_days', '14', 'integer');

-- Create a function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create a trigger to call the function
CREATE TRIGGER update_node_settings_updated_at
BEFORE UPDATE ON node_settings
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
