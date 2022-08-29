USE eth;

-----------------------------------------------------------------
-- CASE: Top NFT within 6hours
-----------------------------------------------------------------
WITH
-- latest 6hours
latest AS
    (
        SELECT min(number)
        FROM   blocks
        WHERE  to_timestamp(timestamp) >
               (
                   SELECT   timeadd(hour, -6, to_timestamp(timestamp))
                   FROM     blocks
                   ORDER BY timestamp DESC limit 1 ))
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
latest AS
    (
        SELECT min(number)
        FROM   blocks
        WHERE  to_timestamp(timestamp) >
               (
                   SELECT   timeadd(hour, -6, to_timestamp(timestamp))
                   FROM     blocks
                   ORDER BY timestamp DESC limit 1 ))
-- select top ens
SELECT    concat(e.NAME, '.eth'),
          e.cost,
          e.owner,
          to_timestamp(b.timestamp) AS date,
          to_timestamp(e.expires)   AS expires,
          transaction_hash
FROM      ens    AS e
    LEFT JOIN blocks AS b
ON        e.block_number=b.number
WHERE     e.block_number >
    (
    SELECT *
    FROM   latest)
ORDER BY  e.cost DESC;