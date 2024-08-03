package delivery

import (
	"net/http"
	"strconv"

	"github.com/labstack/echo/v4"
	"github.com/ritarock/todo-app/model"
	"github.com/ritarock/todo-app/port"
)

type itask struct {
	ID        int64  `json:"id"`
	Text      string `json:"text"`
	Completed bool   `json:"completed"`
}

type taskHandler struct {
	taskUsecase port.TaskUsecase
}

func NewTaskHandler(us port.TaskUsecase) *taskHandler {
	return &taskHandler{
		taskUsecase: us,
	}
}

func (t *taskHandler) Create(c echo.Context) error {
	var task itask
	if err := c.Bind(&task); err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	ctx := c.Request().Context()

	created, err := t.taskUsecase.Create(ctx, model.Task{
		Text:      task.Text,
		Completed: 0,
	})
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	return c.JSON(http.StatusOK, itask{
		ID:        created.ID,
		Text:      created.Text,
		Completed: created.Completed == 1,
	})
}

func (t *taskHandler) GetAll(c echo.Context) error {
	ctx := c.Request().Context()

	tasks, err := t.taskUsecase.GetAll(ctx)
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	result := make([]itask, len(tasks))
	for i, task := range tasks {
		result[i] = itask{task.ID, task.Text, task.Completed == 1}
	}

	return c.JSON(http.StatusOK, result)
}

func (t *taskHandler) GetByID(c echo.Context) error {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	ctx := c.Request().Context()

	task, err := t.taskUsecase.GetByID(ctx, int64(id))
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	return c.JSON(http.StatusOK, itask{
		ID:        task.ID,
		Text:      task.Text,
		Completed: task.Completed == 1,
	})
}

func (t *taskHandler) Update(c echo.Context) error {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	var task itask
	if err := c.Bind(&task); err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}
	var completed int64
	if task.Completed {
		completed = 1
	}

	ctx := c.Request().Context()

	if err := t.taskUsecase.Update(ctx, int64(id), model.Task{Text: task.Text, Completed: completed}); err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	return c.JSON(http.StatusOK, nil)
}

func (t *taskHandler) Delete(c echo.Context) error {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	ctx := c.Request().Context()

	if err := t.taskUsecase.Delete(ctx, int64(id)); err != nil {
		return c.JSON(http.StatusBadRequest, err)
	}

	return c.JSON(http.StatusOK, nil)
}
