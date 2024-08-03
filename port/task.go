package port

import (
	"context"

	"github.com/ritarock/todo-app/model"
)

type TaskRepository interface {
	Create(ctx context.Context, task model.Task) (*model.Task, error)
	GetAll(ctx context.Context) ([]model.Task, error)
	GetByID(ctx context.Context, id int64) (model.Task, error)
	Update(ctx context.Context, id int64, task model.Task) error
	Delete(ctx context.Context, id int64) error
}

type TaskUsecase interface {
	Create(ctx context.Context, task model.Task) (*model.Task, error)
	GetAll(ctx context.Context) ([]model.Task, error)
	GetByID(ctx context.Context, id int64) (model.Task, error)
	Update(ctx context.Context, id int64, task model.Task) error
	Delete(ctx context.Context, id int64) error
}
