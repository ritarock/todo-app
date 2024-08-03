package delivery

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

func NewRouter(taskHandler *taskHandler) *echo.Echo {
	e := echo.New()

	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:5173"},
		AllowHeaders: []string{
			echo.HeaderOrigin,
			echo.HeaderContentType,
			echo.HeaderAccept,
			echo.HeaderAuthorization,
		},
	}))
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.POST("/tasks", taskHandler.Create)
	e.GET("/tasks", taskHandler.GetAll)
	e.GET("/tasks/:id", taskHandler.GetByID)
	e.PUT("tasks/:id", taskHandler.Update)
	e.DELETE("tasks/:id", taskHandler.Delete)

	return e
}
