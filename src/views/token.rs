use actix_web::dev::ServiceRequest;


fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        return Ok(password)
    }
    return Err("token not authorized")
}