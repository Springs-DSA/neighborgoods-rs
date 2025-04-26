-- Drop the item_tags table first to maintain referential integrity
DROP TABLE IF EXISTS item_tags;

-- Drop the tags table after its dependencies have been removed
DROP TABLE IF EXISTS tags;
