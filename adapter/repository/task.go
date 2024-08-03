package repository

import (
	"context"
	"database/sql"

	"github.com/ritarock/todo-app/model"
	"github.com/ritarock/todo-app/port"
)

type taskRepository struct {
	queries model.Queries
}

func NewTaskRepository(db *sql.DB) port.TaskRepository {
	q := model.New(db)
	return &taskRepository{
		queries: *q,
	}
}

func (t *taskRepository) Create(ctx context.Context, task model.Task) (*model.Task, error) {
	created, err := t.queries.CreateTask(ctx, model.CreateTaskParams{
		Text:      task.Text,
		Completed: task.Completed,
	})

	return &created, err
}

func (t *taskRepository) GetAll(ctx context.Context) ([]model.Task, error) {
	return t.queries.ListTasks(ctx)
}

func (t *taskRepository) GetByID(ctx context.Context, id int64) (model.Task, error) {
	return t.queries.GetTask(ctx, id)
}

func (t *taskRepository) Update(ctx context.Context, id int64, task model.Task) error {
	return t.queries.UpdateTask(ctx, model.UpdateTaskParams{
		Text:      task.Text,
		Completed: task.Completed,
		ID:        id,
	})
}

func (t *taskRepository) Delete(ctx context.Context, id int64) error {
	return t.queries.DeleteTask(ctx, id)
}
