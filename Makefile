PHONY: create.model create.mock create.table test wire.build

MOCK_GEN = go run -mod=mod go.uber.org/mock/mockgen@latest

create.model:
	sqlc generate

create.mock:
	$(MOCK_GEN) \
	-source=./port/task.go \
	-destination=./testing/mock/task.go \
	-package=mock

create.table:
	sqlite3 data.sqlite < schema.sql

test:
	go test ./...

wire.build:
	go run github.com/google/wire/cmd/wire@latest ./injector
