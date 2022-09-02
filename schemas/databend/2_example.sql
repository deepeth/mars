USE eth;

-----------------------------------------------------------------
-- CASE: Top NFT within 6hours
-----------------------------------------------------------------
WITH
-- latest 6hours
latest
    AS
    (
        SELECT min(number)
        FROM   blocks
        WHERE  timestamp >
    (
    SELECT   subtract_hours(to_datetime(timestamp), 6)
    FROM     blocks
    ORDER BY timestamp DESC
    LIMIT    1 ))
SELECT   token_address,
         token_id,
         count(*) AS c
FROM     token_transfers
WHERE    block_number>
         (
             SELECT *
             FROM   latest)
  AND      token_id !=''
GROUP BY token_address,
    token_id
ORDER BY c DESC;


-----------------------------------------------------------------
-- CASE: Top ENS within 6hours
-----------------------------------------------------------------
WITH
-- latest 6hours
latest
    AS
    (
        SELECT min(number)
        FROM   blocks
        WHERE  timestamp >
    (
    SELECT   subtract_hours(to_datetime(timestamp), 6)
    FROM     blocks
    ORDER BY timestamp DESC
    LIMIT    1 ))
-- select top ens
SELECT    concat(e.name, '.eth'),
          e.cost,
          e.owner,
          to_datetime(b.timestamp) AS date,
            to_datetime(e.expires)   AS expires,
            transaction_hash
FROM      ens    AS e
    LEFT JOIN blocks AS b
ON        e.block_number=b.number
WHERE     e.block_number >
    (
    SELECT *
    FROM   latest)
ORDER BY  e.cost DESC
    LIMIT     100;