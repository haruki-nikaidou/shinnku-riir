CREATE TABLE IF NOT EXISTS hot_static (
    id BIGSERIAL PRIMARY KEY not null,
    game_id UUID NOT NULL,
    ip VARCHAR(18) NOT NULL,
    download_time TIMESTAMP NOT NULL
);

ALTER TABLE hot_static ALTER COLUMN download_time SET DEFAULT now();

CREATE INDEX IF NOT EXISTS hot_static_game_id_idx ON hot_static(game_id);

CREATE OR REPLACE VIEW top_10_downloaded_games AS
SELECT game_id, COUNT(*)::BIGINT as count
FROM hot_static
GROUP BY game_id
ORDER BY count DESC
LIMIT 10;
