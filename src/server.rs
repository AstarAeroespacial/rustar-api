use crate::{api_doc, routes, state::AppState};
use actix_web::{middleware::Logger, web, App, HttpServer};

/// Configure the Actix Web application
pub fn configure_app(
    app_state: AppState,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(app_state.clone()))
        // Maintain backward compatibility with individual service injections
        .app_data(web::Data::new(app_state.config.clone()))
        .app_data(web::Data::new(app_state.telemetry_service.clone()))
        .app_data(web::Data::new(app_state.ground_station_service.clone()))
        .app_data(web::Data::new(app_state.job_service.clone()))
        .app_data(web::Data::new(app_state.satellite_service.clone()))
        .configure(routes::configure_routes)
        .configure(api_doc::configure_swagger)
        .wrap(Logger::new("%r - %U | %s (%T)"))
}

/// Create and bind the HTTP server
pub fn create_server(
    app_state: AppState,
    address: &str,
) -> std::io::Result<actix_web::dev::Server> {
    let server = HttpServer::new(move || configure_app(app_state.clone())).bind(address)?;

    Ok(server.run())
}

/// Print server startup information
pub fn print_startup_info(server_address: &str) {
    println!("============= API SERVER STARTING =============");
    println!("Available endpoints:");
    println!("  - GET    /api/satellites/{{id}}/telemetry");
    println!("  - GET    /api/config");
    println!("  - POST   /api/control");
    println!("  - POST   /api/jobs");
    println!("  - GET    /api/ground-stations");
    println!("  - GET    /api/ground-stations/{{id}}");
    println!("  - POST   /api/ground-stations");
    println!("  - DELETE /api/ground-stations/{{id}}");
    println!("  - GET    /api/satellites");
    println!("  - GET    /api/satellites/{{id}}");
    println!("  - POST   /api/satellites");
    println!("  - PUT    /api/satellites/{{id}}/tle");
    println!("  - DELETE /api/satellites/{{id}}");
    println!("  - GET    /swagger-ui/");
    println!("Server running at: {}", server_address);
    println!("==============================================");
}
