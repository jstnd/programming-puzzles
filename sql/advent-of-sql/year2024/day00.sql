WITH cities AS (
    SELECT
        city, country
    FROM day00.Children child
    JOIN day00.ChristmasList list ON
        list.child_id = child.child_id AND
        list.was_delivered = TRUE
    GROUP BY
        city, country
    HAVING
        COUNT(*) >= 5
),
city_ranks AS (
    SELECT
        city, country,
        AVG(naughty_nice_score) AS avg_score,
        ROW_NUMBER() OVER (PARTITION BY country ORDER BY AVG(naughty_nice_score) DESC) AS rank
    FROM day00.Children
    GROUP BY
        city, country
)
SELECT DISTINCT
    cities.city
FROM cities
LEFT JOIN city_ranks ON
    city_ranks.city = cities.city
WHERE
    city_ranks.rank <= 3;