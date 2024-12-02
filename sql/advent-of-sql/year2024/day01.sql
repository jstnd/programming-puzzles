SELECT
    CONCAT_WS(',',
        children.name,
        JSON_EXTRACT(wl.wishes, '$.first_choice'),
        JSON_EXTRACT(wl.wishes, '$.second_choice'),
        JSON_EXTRACT(wl.wishes, '$.colors[0]'),
        JSON_ARRAY_LENGTH(JSON_EXTRACT(wl.wishes, '$.colors')),
        CASE
            WHEN tc.difficulty_to_make = 1 THEN 'Simple Gift'
            WHEN tc.difficulty_to_make = 2 THEN 'Moderate Gift'
            WHEN tc.difficulty_to_make >= 3 THEN 'Complex Gift'
        END,
        CASE tc.category
            WHEN 'outdoor' THEN 'Outside Workshop'
            WHEN 'educational' THEN 'Learning Workshop'
            ELSE 'General Workshop'
        END
    ) AS result
FROM day01.children children
JOIN day01.wish_lists wl ON
    wl.child_id = children.child_id
JOIN day01.toy_catalogue tc ON
    tc.toy_name = JSON_EXTRACT(wl.wishes, '$.first_choice')
ORDER BY
    children.name
LIMIT 5