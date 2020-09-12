-- Contextual information such as the commit hash used for building the cache,
-- the version of Onelo used, etc.
CREATE TABLE IF NOT EXISTS context (
    key   text NOT NULL PRIMARY KEY,
    value text NOT NULL
);

-- The set of sources with their routes. Identifiers are expected to be used
-- in source entries to compact the path.
--
-- Routes can be from the local filesystem or from remote sources such as a
-- Git repository.
CREATE TABLE IF NOT EXISTS source (
    id        text NOT NULL PRIMARY KEY,
    route     text NOT NULL,
    timestamp datetime NOT NULL
);

-- The set of source entries found in the processed sources.
CREATE TABLE IF NOT EXISTS source_entry (
  id       text NOT NULL PRIMARY KEY,
  checksum text NOT NULL,
  content  blob NOT NULL,
  title    text NOT NULL
);
