-- https://github.com/nalgeon/sqlean
SELECT LOAD_EXTENSION('C:\Users\justi\.sqlite\regexp.dll');

SELECT
    REGEXP_REPLACE(
        GROUP_CONCAT(letters.letter, ''),
        '[^a-zA-Z!"''(),-.:;? ]',
        ''
    )
FROM (
    SELECT
        GROUP_CONCAT(CHAR(value), '') AS letter
    FROM day02.letters_a
    UNION
    SELECT
        GROUP_CONCAT(CHAR(value), '') AS letter
    FROM day02.letters_b
) letters;