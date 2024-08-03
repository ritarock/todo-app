package delivery

import "github.com/labstack/echo/v4"

func NewRouter(taskHandler *taskHandler) *echo.Echo {
	e := echo.New()

	e.POST("/tasks", taskHandler.Create)
	e.GET("/tasks", taskHandler.GetAll)
	e.GET("/tasks/:id", taskHandler.GetByID)
	e.PUT("tasks/:id", taskHandler.Update)
	e.DELETE("tasks/:id", taskHandler.Delete)

	return e
}
