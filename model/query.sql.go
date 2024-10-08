// Code generated by sqlc. DO NOT EDIT.
// versions:
//   sqlc v1.26.0
// source: query.sql

package model

import (
	"context"
)

const createTask = `-- name: CreateTask :one
INSERT INTO tasks (
    text, completed
) VALUES (
    ?, ?
)
RETURNING id, text, completed
`

type CreateTaskParams struct {
	Text      string
	Completed int64
}

func (q *Queries) CreateTask(ctx context.Context, arg CreateTaskParams) (Task, error) {
	row := q.db.QueryRowContext(ctx, createTask, arg.Text, arg.Completed)
	var i Task
	err := row.Scan(&i.ID, &i.Text, &i.Completed)
	return i, err
}

const deleteTask = `-- name: DeleteTask :exec
DELETE FROM tasks
WHERE id = ?
`

func (q *Queries) DeleteTask(ctx context.Context, id int64) error {
	_, err := q.db.ExecContext(ctx, deleteTask, id)
	return err
}

const getTask = `-- name: GetTask :one
SELECT id, text, completed FROM tasks
WHERE id = ?
`

func (q *Queries) GetTask(ctx context.Context, id int64) (Task, error) {
	row := q.db.QueryRowContext(ctx, getTask, id)
	var i Task
	err := row.Scan(&i.ID, &i.Text, &i.Completed)
	return i, err
}

const listTasks = `-- name: ListTasks :many
SELECT id, text, completed FROM tasks
ORDER BY id
`

func (q *Queries) ListTasks(ctx context.Context) ([]Task, error) {
	rows, err := q.db.QueryContext(ctx, listTasks)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var items []Task
	for rows.Next() {
		var i Task
		if err := rows.Scan(&i.ID, &i.Text, &i.Completed); err != nil {
			return nil, err
		}
		items = append(items, i)
	}
	if err := rows.Close(); err != nil {
		return nil, err
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return items, nil
}

const updateTask = `-- name: UpdateTask :exec
UPDATE tasks SET
text = ?, completed = ?
WHERE id = ?
RETURNING id, text, completed
`

type UpdateTaskParams struct {
	Text      string
	Completed int64
	ID        int64
}

func (q *Queries) UpdateTask(ctx context.Context, arg UpdateTaskParams) error {
	_, err := q.db.ExecContext(ctx, updateTask, arg.Text, arg.Completed, arg.ID)
	return err
}
