An API for listening to webhook events from helius, and manages workflows.

## Routes
| Method | Endpoint                  | Description                                                                      |
| ------ | ------------------------- | -------------------------------------------------------------------------------- |
| GET    | /health                   | Ensures the API is up and running                                                |
| POST   | /webhook                  | Recieves a new webhook event and saves it to the trigger_events table            |
| GET    | /events                   | Retrieves all webhook events that have been recieved                             |
| GET    | /events/{wallet}          | Retrieves all webhook events that have been received for a particular wallet     |
| GET    | /events/mint/{token_mint} | Retrieves all webhook events that have been received for a particular token mint |
| POST   | /workflows                | Receives a new workflow and saves it to the workflows table                      |
| GET    | /workflows                | Retrieves all workflows                                                          |
| GET    | /workflows/active         | Retrieves all active workflows                                                   |
| POST   | /workflows/{id}/pause     | Changes the status of a workflow to `paused`                                     |
| POST   | /workflows/{id}/activate  | Changes the status of a workflow to `active`                                     |

## Setup
1. Clone the repo
2. Setup env variables, RUST_LOG, and DATABASE_URL in an `.env` file
		```
			DATABASE_URL=sqlite:webhooks.db?mode=rwc
			RUST_LOG=debug
		```
3. Run
		```
		cargo run
		```

## Testing
Tests are in the /tests folder, and use an in memory sqlite db. To test, run:
```bash
cargo test
```

## TBA
Features tba include rate limiting &/ auth 
