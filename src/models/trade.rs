use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::book::GoogleBookDto;
use crate::models::user::UserResponse;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PossibleTrade {
    pub offered_book: GoogleBookDto,
    #[schema(value_type = String, format = "uuid")]
    pub offered_book_id: Uuid,
    pub wanted_book: GoogleBookDto,
    #[schema(value_type = String, format = "uuid")]
    pub wanted_book_id: Uuid,
    pub trade_partner: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradeMatch {
    #[schema(value_type = String, format = "uuid")]
    pub user_offers: Uuid,  // ID do livro que o usuário oferece
    #[schema(value_type = String, format = "uuid")]
    pub user_wants: Uuid,   // ID do livro que o usuário quer
    #[schema(value_type = String, format = "uuid")]
    pub partner_offers: Uuid, // ID do livro que o parceiro oferece
    #[schema(value_type = String, format = "uuid")]
    pub partner_wants: Uuid,  // ID do livro que o parceiro quer
    pub partner: UserResponse,
} 