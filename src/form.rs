use serde::Serialize;
use uuid::Uuid;

use crate::constants;

#[derive(Serialize)]
pub struct DeleteAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: u32,
	gjp: &'a str,
	#[serde(rename = "commentID")]
	comment_id: u32,
	secret: &'static str,
}

pub fn delete_account_comment(account_id: u32, gjp: &str, comment_id: u32) -> DeleteAccountCommentForm {
	DeleteAccountCommentForm {
		account_id,
		gjp,
		comment_id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct GauntletForm {
	gauntlet: u8,
	secret: &'static str,
}

pub fn gauntlet(id: u8) -> GauntletForm {
	GauntletForm {
		gauntlet: id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct GauntletsForm {
	special: u8,
	secret: &'static str,
}

pub fn gauntlets() -> GauntletsForm {
	GauntletsForm {
		special: 1,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct LevelForm {
	#[serde(rename = "levelID")]
	level_id: u32,
	secret: &'static str,
}

pub fn level(id: u32) -> LevelForm {
	LevelForm {
		level_id: id,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct LevelsForm {
	str: String,
	r#type: u8,
	secret: &'static str,
}

pub fn levels(ids: &[u32]) -> LevelsForm {
	LevelsForm {
		str: ids.iter().map(|e| e.to_string() + ",").collect(),
		r#type: 10,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct LoginForm<'a> {
	#[serde(rename = "userName")]
	username: &'a str,
	password: &'a str,
	udid: Uuid,
	secret: &'static str,
}

pub fn login<'a>(username: &'a str, password: &'a str) -> LoginForm<'a> {
	LoginForm {
		username,
		password,
		udid: Uuid::new_v4(),
		secret: constants::LOGIN_SECRET,
	}
}

#[derive(Serialize)]
pub struct MapPacksForm {
	page: u8,
	secret: &'static str,
}

pub fn map_packs(page: u8) -> MapPacksForm {
	MapPacksForm {
		page,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct SearchUserForm<'a> {
	str: &'a str,
	secret: &'static str,
}

pub fn search_user(username: &str) -> SearchUserForm {
	SearchUserForm {
		str: username,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct UploadAccountCommentForm<'a> {
	#[serde(rename = "accountID")]
	account_id: u32,
	gjp: &'a str,
	comment: &'a str,
	secret: &'static str,
}

pub fn upload_account_comment<'a>(account_id: u32, gjp: &'a str, comment: &'a str) -> UploadAccountCommentForm<'a> {
	UploadAccountCommentForm {
		account_id,
		gjp,
		comment,
		secret: constants::SECRET,
	}
}

#[derive(Serialize)]
pub struct UserForm {
	#[serde(rename = "targetAccountID")]
	target_account_id: u32,
	secret: &'static str,
}

pub fn user(account_id: u32) -> UserForm {
	UserForm {
		target_account_id: account_id,
		secret: constants::SECRET,
	}
}
