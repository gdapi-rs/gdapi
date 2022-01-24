use crate::{builders::*, constants, data::*, error::*, form, parser::*};
use gdapi_crypto::encode;
use reqwest::Client as ReqwestClient;
use serde::Serialize;

#[derive(Clone, Debug, Default)]
pub struct Client {
	account_id: Option<u32>,
	gjp: Option<String>,
	inner: ReqwestClient,
}

impl Client {
	pub fn new() -> Self {
		Self::default()
	}

	pub(crate) async fn request<T: APIData>(&self, endpoint: &str, form: impl Serialize) -> Result<T> {
		let url = format!("{}/{}.php", constants::BASE_URL, endpoint);
		let response = self.inner.post(url).form(&form).send().await?.text().await?;

		if response == "-1" {
			Err(Error::InvalidRequest)
		} else {
			T::parse_data(&response).ok_or(Error::ParseResponse)
		}
	}

	pub async fn gauntlets(&self) -> Result<Vec<Gauntlet>> {
		self.request("getGJGauntlets21", form::gauntlets()).await
	}

	pub async fn level(&self, id: u32) -> Result<Level> {
		self.request("downloadGJLevel22", form::level(id)).await
	}

	pub async fn login(&mut self, username: &str, password: &str) -> Result<LoginResponse> {
		let data: LoginResponse = self
			.request("accounts/loginGJAccount", form::login(username, password))
			.await?;

		self.account_id = Some(data.account_id);
		self.gjp = Some(encode::gjp(password)?);

		Ok(data)
	}

	pub fn map_packs(&self) -> MapPacksBuilder {
		MapPacksBuilder::new(self)
	}

	pub async fn search_user(&self, username: &str) -> Result<User> {
		self.request("getGJUsers20", form::search_user(username)).await
	}

	pub async fn upload_account_comment(&self, comment: &str) -> Result<u32> {
		let account_id = self.account_id.ok_or(Error::NotLoggedIn)?;
		let gjp = self.gjp.as_ref().ok_or(Error::NotLoggedIn)?;
		let comment = encode::base64(comment);
		let form = form::upload_account_comment(account_id, gjp, &comment);

		self.request("uploadGJAccComment20", form).await
	}

	pub async fn user(&self, account_id: u32) -> Result<User> {
		self.request("getGJUserInfo20", form::user(account_id)).await
	}
}
