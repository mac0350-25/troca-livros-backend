#[utoipa::path(
    post,
    path = "/api/books/search",
    request_body = BookSearchRequest,
    responses(
        (status = 200, description = "Busca de livros realizada com sucesso", body = Vec<GoogleBookDto>),
        (status = 400, description = "Erro de validação", body = String),
        (status = 500, description = "Erro interno do servidor", body = String)
    ),
    tag = "books"
)]
#[allow(unused)]
pub fn search_books() {
    unimplemented!()
}
