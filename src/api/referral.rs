use crate::client::BybitClient;
use crate::error::Result;
use crate::models::referral::*;

impl BybitClient {
    pub async fn get_referral_campaign_rewards(
        &self,
        params: GetReferralCampaignRewardsParams,
    ) -> Result<GetReferralCampaignRewardsResponse> {
        self.post("/v5/referral/campaign/rewards", &params).await
    }

    pub async fn get_referral_commission_records(
        &self,
        params: GetReferralCommissionRecordsParams,
    ) -> Result<GetReferralCommissionRecordsResponse> {
        self.post("/v5/referral/commission/records", &params).await
    }

    pub async fn get_referral_user_identity_profile(
        &self,
        params: GetReferralUserIdentityProfileParams,
    ) -> Result<GetReferralUserIdentityProfileResponse> {
        self.post("/v5/referral/user/identity-profile", &params)
            .await
    }

    pub async fn get_referral_user_invitations(
        &self,
        params: GetReferralUserInvitationsParams,
    ) -> Result<GetReferralUserInvitationsResponse> {
        self.post("/v5/referral/user/invitations", &params).await
    }
}
