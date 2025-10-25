Satellite Management
GET /api/satellites - Get all satellites with TLE data
GET /api/satellites/{id} - Get specific satellite by ID
PUT /api/satellites/{id} - Update satellite TLE data
GET /api/satellite/{id}/commands - Get available commands

Ground Station Management
GET /api/ground-stations - Get all ground stations
GET /api/ground-stations/{id} - Get specific ground station
POST /api/ground-stations - Create new ground station
PUT /api/ground-stations/{id}/satellite - Update station's tracking satellite

Telemetry
GET /api/satellite/{id}/telemetry - Fetch (decoded) telemetry
    pageSize: int
    pageNumber: int

Tracking
GET /api/satellites/{id}/passes - Get next passes for ground stations
GET /api/ground-stations/{id}/passes - Get next satellites to observe
POST /api/ground-stations/{id}/passes - Get next satellites to observe
    [sat_ids]

===
GET /api/satellite/{id}/telemetry/decoder
PUT /api/satellite/{id}/telemetry/decoder
===


Jobs
POST /api/jobs - Create job
    {
        gs_id: string,
        sat_id: string,
        commands: [commands (enum)]
    }
GET /api/jobs - Get all jobs
GET /api/jobs/{id} - Get specific job
GET /jobs/{id}/status - Get job status

