## Satellite Management
- [x] GET /api/satellites - Get all satellites
- [x] POST /api/satellites - Create new satellite
- [x] GET /api/satellites/{id} - Get specific satellite by ID
- [x] PUT /api/satellites/{id}/tle - Update satellite TLE data
- [x] DELETE /api/satellites/{id} - Delete satellite by ID
- [ ] GET /api/satellite/{id}/commands - Get available commands

## Ground Station Management
- [x] GET /api/ground-stations - Get all ground stations
- [x] GET /api/ground-stations/{id} - Get specific ground station
- [x] POST /api/ground-stations - Create new ground station
- [x] DELETE /api/ground-stations/{id} - Delete ground station by ID

## Telemetry
- [x] GET /api/satellite/{id}/telemetry - Fetch (decoded) telemetry
    page: int
    limit: int

## Tracking
- [ ] GET /api/satellites/{id}/passes - Get next passes over ground stations
- [ ] GET /api/ground-stations/{id}/passes - Get next satellite observations

## Jobs
- [ ] POST /api/jobs - Create job
    {
        gs_id: string,
        sat_id: string,
        commands: [commands (enum)]
    }
- [ ] GET /api/jobs - Get all jobs
- [ ] GET /api/jobs/{id} - Get specific job
- [ ] GET /jobs/{id}/status - Get job status
