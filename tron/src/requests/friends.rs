use crate::BridgeService;
use crate::bridge::{
    AcceptFriendRequestRequest, AcceptFriendRequestResponse, GetFriendRequestsRequest,
    GetFriendRequestsResponse, GetFriendsRequest, GetFriendsResponse, RejectFriendRequestRequest,
    RejectFriendRequestResponse, SendFriendRequestRequest, SendFriendRequestResponse,
};
use tonic::{Request, Response, Status};

impl BridgeService {
    pub async fn handle_get_friends(
        &self,
        request: Request<GetFriendsRequest>,
    ) -> Result<Response<GetFriendsResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_get_friend_requests(
        &self,
        request: Request<GetFriendRequestsRequest>,
    ) -> Result<Response<GetFriendRequestsResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_send_friend_request(
        &self,
        request: Request<SendFriendRequestRequest>,
    ) -> Result<Response<SendFriendRequestResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_accept_friend_request(
        &self,
        request: Request<AcceptFriendRequestRequest>,
    ) -> Result<Response<AcceptFriendRequestResponse>, Status> {
        unimplemented!()
    }

    pub async fn handle_reject_friend_request(
        &self,
        request: Request<RejectFriendRequestRequest>,
    ) -> Result<Response<RejectFriendRequestResponse>, Status> {
        unimplemented!()
    }
}
