package usecase

import (
	"context"
	"time"

	"github.com/ritarock/todo-app/model"
	"github.com/ritarock/todo-app/port"
)

type taskUsecase struct {
	taskRepo       port.TaskRepository
	contextTimeout time.Duration
}

func NewTaskUsecase(repo port.TaskRepository, timeout time.Duration) port.TaskUsecase {
	return &taskUsecase{
		taskRepo:       repo,
		contextTimeout: timeout,
	}
}

func (t *taskUsecase) Create(ctx context.Context, task model.Task) (*model.Task, error) {
	ctx, cancel := context.WithTimeout(ctx, t.contextTimeout)
	defer cancel()

	return t.taskRepo.Create(ctx, task)
}

func (t *taskUsecase) GetAll(ctx context.Context) ([]model.Task, error) {
	ctx, cancel := context.WithTimeout(ctx, t.contextTimeout)
	defer cancel()

	return t.taskRepo.GetAll(ctx)
}

func (t *taskUsecase) GetByID(ctx context.Context, id int64) (model.Task, error) {
	ctx, cancel := context.WithTimeout(ctx, t.contextTimeout)
	defer cancel()

	return t.taskRepo.GetByID(ctx, id)
}

func (t *taskUsecase) Update(ctx context.Context, id int64, task model.Task) error {
	ctx, cancel := context.WithTimeout(ctx, t.contextTimeout)
	defer cancel()

	return t.taskRepo.Update(ctx, id, task)
}

func (t *taskUsecase) Delete(ctx context.Context, id int64) error {
	ctx, cancel := context.WithTimeout(ctx, t.contextTimeout)
	defer cancel()

	return t.taskRepo.Delete(ctx, id)
}
