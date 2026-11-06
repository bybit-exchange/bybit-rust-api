//! User API endpoints.

use crate::client::BybitClient;
use crate::error::Result;
use crate::models::user::*;

impl BybitClient {
    /// Get affiliate sub-affiliate list.
    pub async fn get_affiliate_sub_list(
        &self,
        cursor: Option<&str>,
        size: Option<i64>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        sub_aff_id: Option<i64>,
    ) -> Result<GetAffiliateSubListResponse> {
        let size_str;
        let sub_aff_id_str;
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        if let Some(s) = size {
            size_str = s.to_string();
            params.push(("size", size_str.as_str()));
        }
        if let Some(s) = start_date {
            params.push(("startDate", s));
        }
        if let Some(e) = end_date {
            params.push(("endDate", e));
        }
        if let Some(s) = sub_aff_id {
            sub_aff_id_str = s.to_string();
            params.push(("subAffId", sub_aff_id_str.as_str()));
        }
        self.get("/v5/affiliate/affiliate-sub-list", &params).await
    }

    /// Get single direct client user info.
    pub async fn get_affiliate_user_info(&self, uid: &str) -> Result<GetAffiliateUserInfoResponse> {
        let params = vec![("uid", uid)];
        self.get("/v5/user/aff-customer-info", &params).await
    }

    /// Get affiliate user list.
    #[allow(clippy::too_many_arguments)] // TODO(api-ergonomics): convert positional args to a typed `*Params` struct
    pub async fn get_affiliate_user_list(
        &self,
        cursor: Option<&str>,
        size: Option<i64>,
        need_deposit: Option<bool>,
        need30: Option<bool>,
        need365: Option<bool>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<GetAffiliateUserListResponse> {
        let size_str;
        let need_deposit_str;
        let need30_str;
        let need365_str;
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        if let Some(s) = size {
            size_str = s.to_string();
            params.push(("size", size_str.as_str()));
        }
        if let Some(n) = need_deposit {
            need_deposit_str = n.to_string();
            params.push(("needDeposit", need_deposit_str.as_str()));
        }
        if let Some(n) = need30 {
            need30_str = n.to_string();
            params.push(("need30", need30_str.as_str()));
        }
        if let Some(n) = need365 {
            need365_str = n.to_string();
            params.push(("need365", need365_str.as_str()));
        }
        if let Some(s) = start_date {
            params.push(("startDate", s));
        }
        if let Some(e) = end_date {
            params.push(("endDate", e));
        }
        self.get("/v5/affiliate/aff-user-list", &params).await
    }

    /// Create Sub API Key.
    pub async fn create_sub_api_key(
        &self,
        params: CreateSubApiKeyParams,
    ) -> Result<CreateSubApiKeyResponse> {
        self.post("/v5/user/create-sub-api", &params).await
    }

    /// Create Sub UID.
    pub async fn create_sub_member(
        &self,
        params: CreateSubMemberParams,
    ) -> Result<CreateSubMemberResponse> {
        self.post("/v5/user/create-sub-member", &params).await
    }

    /// Delete Master API Key.
    pub async fn delete_api_key(&self, params: DeleteApiKeyParams) -> Result<DeleteApiKeyResponse> {
        self.post("/v5/user/delete-api", &params).await
    }

    /// Delete Sub-account API Key.
    pub async fn delete_sub_api_key(
        &self,
        params: DeleteSubApiKeyParams,
    ) -> Result<DeleteSubApiKeyResponse> {
        self.post("/v5/user/delete-sub-api", &params).await
    }

    /// Delete Sub-account.
    pub async fn delete_sub_member_v5(
        &self,
        params: DeleteSubMemberV5Params,
    ) -> Result<DeleteSubMemberV5Response> {
        self.post("/v5/user/del-submember", &params).await
    }

    /// Freeze/Unfreeze Sub UID.
    pub async fn frozen_sub_member(
        &self,
        params: FrozenSubMemberParams,
    ) -> Result<FrozenSubMemberResponse> {
        self.post("/v5/user/frozen-sub-member", &params).await
    }

    /// Get Member Account Type.
    // FIXME(typed-signature): falls back to `serde_json::Value` because the
    // OpenAPI spec referenced a response/request type that gen-sdk-rust could
    // not auto-resolve. Replace with a typed struct in a follow-up PR.
    pub async fn get_member_account_type(
        &self,
        member_ids: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(m) = member_ids {
            params.push(("memberIds", m));
        }
        self.get("/v5/user/get-member-type", &params).await
    }

    /// List Sub-account API Keys.
    ///
    /// See: https://bybit-exchange.github.io/docs/v5/user/list-sub-apikeys
    pub async fn list_sub_api_keys_v5(
        &self,
        sub_member_id: i64,
        limit: Option<i64>,
        cursor: Option<&str>,
    ) -> Result<ListSubApiKeysV5Response> {
        let sub_member_id_str = sub_member_id.to_string();
        let limit_str;
        let mut params: Vec<(&str, &str)> = vec![("subMemberId", sub_member_id_str.as_str())];
        if let Some(l) = limit {
            limit_str = l.to_string();
            params.push(("limit", limit_str.as_str()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        self.get("/v5/user/sub-apikeys", &params).await
    }

    /// Get API Key Information.
    pub async fn query_api_key(&self) -> Result<QueryApiKeyResponse> {
        self.get("/v5/user/query-api", &[]).await
    }

    /// Query Escrow Sub-accounts (Fund Management).
    pub async fn query_escrow_sub_members_v5(
        &self,
        next_cursor: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<QueryEscrowSubMembersV5Response> {
        let next_cursor_str;
        let page_size_str;
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(n) = next_cursor {
            next_cursor_str = n.to_string();
            params.push(("nextCursor", next_cursor_str.as_str()));
        }
        if let Some(p) = page_size {
            page_size_str = p.to_string();
            params.push(("pageSize", page_size_str.as_str()));
        }
        self.get("/v5/user/escrow_sub_members", &params).await
    }

    /// Query Referrals.
    pub async fn query_referrals(
        &self,
        cursor: Option<&str>,
        size: Option<i64>,
        status: Option<&str>,
    ) -> Result<QueryReferralsResponse> {
        let size_str;
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c));
        }
        if let Some(s) = size {
            size_str = s.to_string();
            params.push(("size", size_str.as_str()));
        }
        if let Some(s) = status {
            params.push(("status", s));
        }
        self.get("/v5/user/invitation/referrals", &params).await
    }

    /// Query Sub UID List.
    pub async fn query_sub_members(&self) -> Result<QuerySubMembersResponse> {
        self.get("/v5/user/query-sub-members", &[]).await
    }

    /// Query Sub-accounts List.
    pub async fn query_sub_members_v5(
        &self,
        page_size: Option<i64>,
        next_cursor: Option<i64>,
    ) -> Result<QuerySubMembersV5Response> {
        let page_size_str;
        let next_cursor_str;
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(p) = page_size {
            page_size_str = p.to_string();
            params.push(("pageSize", page_size_str.as_str()));
        }
        if let Some(n) = next_cursor {
            next_cursor_str = n.to_string();
            params.push(("nextCursor", next_cursor_str.as_str()));
        }
        self.get("/v5/user/submembers", &params).await
    }

    /// Sign Agreement.
    pub async fn sign_agreement(
        &self,
        params: SignAgreementParams,
    ) -> Result<SignAgreementResponse> {
        self.post("/v5/user/agreement", &params).await
    }

    /// Modify Master API Key.
    pub async fn update_api_key(&self, params: UpdateApiKeyParams) -> Result<UpdateApiKeyResponse> {
        self.post("/v5/user/update-api", &params).await
    }

    /// Modify Sub-account API Key.
    pub async fn update_sub_api_key(
        &self,
        params: UpdateSubApiKeyParams,
    ) -> Result<UpdateSubApiKeyResponse> {
        self.post("/v5/user/update-sub-api", &params).await
    }
}

impl BybitClient {
    pub async fn query_referral_code(&self) -> Result<QueryReferralCodeResponse> {
        let params = vec![];
        self.get("/v5/user/invitation/code", &params).await
    }
}
