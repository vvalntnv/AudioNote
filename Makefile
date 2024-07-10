down:
	docker compose -f docker/compose.yaml down

run:
	docker compose -f docker/compose.yaml up -d

shell-postgres:
	docker exec -it audio-postgres bash

shell-mongo:
	docker exec -it audio-mongo bash
