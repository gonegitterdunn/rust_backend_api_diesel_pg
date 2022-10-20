use actix_web::{get, HttpResponse};

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(|| Employees::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employee/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = web::block(|| Employees::find(id.into_inner()))
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = web::block(|| Employees::create(employee.into_inner()))
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Employee>,
) -> Result<HttpResponse, CustomError> {
    let employee = web::block(|| Employees::update(id.into_inner(), employee, into_inner()))
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = web::block(|| Employees::delete(id.into_inner()))
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
