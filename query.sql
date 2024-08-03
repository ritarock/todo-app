-- name: CreateTask :one
INSERT INTO tasks (
    text, completed
) VALUES (
    ?, ?
)
RETURNING *;

-- name: ListTasks :many
SELECT * FROM tasks
ORDER BY id;

-- name: GetTask :one
SELECT * FROM tasks
WHERE id = ?;

-- name: UpdateTask :exec
UPDATE tasks SET
text = ?, completed = ?
WHERE id = ?
RETURNING *;

-- name: DeleteTask :exec
DELETE FROM tasks
WHERE id = ?;
