## Satellite Management
- GET /api/satellites - Get all satellites
- POST /api/satellites - Create new satellite
- GET /api/satellites/{id} - Get specific satellite by ID
- PUT /api/satellites/{id}/tle - Update satellite TLE data
- DELETE /api/satellites/{id} - Delete satellite by ID
- GET /api/satellite/{id}/commands - Get available commands

## Ground Station Management
- GET /api/ground-stations - Get all ground stations
- GET /api/ground-stations/{id} - Get specific ground station
- POST /api/ground-stations - Create new ground station
- DELETE /api/ground-stations/{id} - Delete ground station by ID

## Telemetry
- GET /api/satellite/{id}/telemetry - Fetch (decoded) telemetry
    page: int
    limit: int

## Tracking
- GET /api/satellites/{id}/passes - Get next passes over ground stations
- GET /api/ground-stations/{id}/passes - Get next satellite observations

## Jobs
- POST /api/jobs - Create job
    {
        gs_id: string,
        sat_id: string,
        commands: [commands (enum)]
    }
- GET /api/jobs - Get all jobs
- GET /api/jobs/{id} - Get specific job
- GET /jobs/{id}/status - Get job status
