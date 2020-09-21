-- Contextual information such as the version of Onelo used, etc.
CREATE TABLE IF NOT EXISTS context (
    key   text NOT NULL PRIMARY KEY,
    value text NOT NULL
);

-- The set of known content types.
CREATE TABLE IF NOT EXISTS content_type (
    -- Not all possible media types are registered but when they are, they
    -- must match the IANA [registry](https://www.iana.org/assignments/media-types/media-types.xhtml)
    id text NOT NULL PRIMARY KEY
);

-- The set of sources with their routes. Identifiers are expected to be used
-- in source entries to compact the path.
--
-- Routes can be from the local filesystem or from remote sources such as a
-- Git repository.
CREATE TABLE IF NOT EXISTS source (
    id        text NOT NULL PRIMARY KEY,
    route     text NOT NULL,
    checksum  text,
    timestamp datetime NOT NULL
);

-- The set of source entries found in the processed sources.
CREATE TABLE IF NOT EXISTS source_entry (
    id              text NOT NULL PRIMARY KEY,
    source_id       text NOT NULL,
    content_id      text NOT NULL,
    content_type_id text,

    FOREIGN KEY (source_id) REFERENCES source(id),
    FOREIGN KEY (content_type_id) REFERENCES content_type(id),
    FOREIGN KEY (content_id) REFERENCES content(id)
);

-- The set of content across sources.
CREATE TABLE IF NOT EXISTS content (
    -- content checksum
    id              text NOT NULL PRIMARY KEY,
    content         blob NOT NULL
);
