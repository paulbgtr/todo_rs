use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let welcoming_message = r#"
        Welcome to the Rust API!
        
        This API provides endpoints to manage your tasks and todos.
        You can create, retrieve, update, and delete todos using the provided endpoints.
        
        API Endpoints:
        - GET /todos: Retrieve all todos
        - POST /todos: Create a new todo
        - GET /todos/{id}: Retrieve a specific todo
        - PUT /todos/{id}: Update a todo
        - DELETE /todos/{id}: Delete a todo
        
        Please refer to the API documentation for detailed information on request/response structures and usage.
    "#;

    HttpResponse::Ok().body(welcoming_message)
}

#[get("/todos")]
async fn get_all_todos() -> impl Responder {
    HttpResponse::Ok().body("todos")
}

#[post("/todos")]
async fn create_todo() -> impl Responder {
    HttpResponse::Ok().body("todos")
}

#[get("/todos/{id}")]
async fn get_todo(path: web::Path<(u32)>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body("todos")
}

#[put("/todos/{id}")]
async fn update_todo() -> impl Responder {
    HttpResponse::Ok().body("todos")
}

#[delete("/todos/{id}")]
async fn delete_todo() -> impl Responder {
    HttpResponse::Ok().body("todos")
}
