//go:build wireinject
// +build wireinject

package injector

import (
	"database/sql"
	"time"

	"github.com/google/wire"
	"github.com/labstack/echo/v4"
	"github.com/ritarock/todo-app/adapter/delivery"
	"github.com/ritarock/todo-app/adapter/repository"
	"github.com/ritarock/todo-app/usecase"
)

func Initialize(db *sql.DB, timeout time.Duration) (*echo.Echo, error) {
	wire.Build(
		repository.NewTaskRepository,
		usecase.NewTaskUsecase,
		delivery.NewTaskHandler,
		delivery.NewRouter,
	)

	return nil, nil
}
