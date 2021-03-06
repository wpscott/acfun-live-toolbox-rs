use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .protoc()
        .includes(&["src/protos/Im/Basic", "src/protos/zt.live.interactive"])
        .inputs([
            "src/protos/Im/Basic/AccessPoint.proto",
            "src/protos/Im/Basic/AccessPointsConfig.proto",
            "src/protos/Im/Basic/AppInfo.proto",
            "src/protos/Im/Basic/DeviceInfo.proto",
            "src/protos/Im/Basic/DownstreamPayload.proto",
            "src/protos/Im/Basic/EnvInfo.proto",
            "src/protos/Im/Basic/ErrorMessage.proto",
            "src/protos/Im/Basic/FrontendInfo.proto",
            "src/protos/Im/Basic/HandshakeRequest.proto",
            "src/protos/Im/Basic/HandshakeResponse.proto",
            "src/protos/Im/Basic/I18nCopyWriting.proto",
            "src/protos/Im/Basic/KeepAliveRequest.proto",
            "src/protos/Im/Basic/KeepAliveResponse.proto",
            "src/protos/Im/Basic/LinkErrorCode.proto",
            "src/protos/Im/Basic/LocaleMessage.proto",
            "src/protos/Im/Basic/PacketHeader.proto",
            "src/protos/Im/Basic/PingRequest.proto",
            "src/protos/Im/Basic/PingResponse.proto",
            "src/protos/Im/Basic/PushServiceToken.proto",
            "src/protos/Im/Basic/RegisterRequest.proto",
            "src/protos/Im/Basic/RegisterResponse.proto",
            "src/protos/Im/Basic/RequsetBasicInfo.proto",
            "src/protos/Im/Basic/SdkOption.proto",
            "src/protos/Im/Basic/SettingInfo.proto",
            "src/protos/Im/Basic/SharePlatform.proto",
            "src/protos/Im/Basic/SyncCookie.proto",
            "src/protos/Im/Basic/TokenInfo.proto",
            "src/protos/Im/Basic/UnregisterRequest.proto",
            "src/protos/Im/Basic/UnregisterResponse.proto",
            "src/protos/Im/Basic/UpstreamPayload.proto",
            "src/protos/Im/Basic/User.proto",
            "src/protos/Im/Basic/UserInstance.proto",
            "src/protos/Im/Basic/ZtCommonInfo.proto",
        ])
        .inputs([
            "src/protos/zt.live.interactive/acfun.live.proto",
            "src/protos/zt.live.interactive/AuthorChatPlayerInfo.proto",
            "src/protos/zt.live.interactive/BackgroundStyle.proto",
            "src/protos/zt.live.interactive/Button.proto",
            "src/protos/zt.live.interactive/ChatMediaType.proto",
            "src/protos/zt.live.interactive/ClickEvent.proto",
            "src/protos/zt.live.interactive/CommentNotice.proto",
            "src/protos/zt.live.interactive/CommonActionSignalComment.proto",
            "src/protos/zt.live.interactive/CommonActionSignalGift.proto",
            "src/protos/zt.live.interactive/CommonActionSignalLike.proto",
            "src/protos/zt.live.interactive/CommonActionSignalRichText.proto",
            "src/protos/zt.live.interactive/CommonActionSignalUserEnterRoom.proto",
            "src/protos/zt.live.interactive/CommonActionSignalUserFollowAuthor.proto",
            "src/protos/zt.live.interactive/CommonActionSignalUserShareLive.proto",
            "src/protos/zt.live.interactive/CommonNotifySignalCoverAuditResult.proto",
            "src/protos/zt.live.interactive/CommonNotifySignalKickedOut.proto",
            "src/protos/zt.live.interactive/CommonNotifySignalLiveManagerState.proto",
            "src/protos/zt.live.interactive/CommonNotifySignalRemoveApplyUser.proto",
            "src/protos/zt.live.interactive/CommonNotifySignalViolationAlert.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorChatAccept.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorChatCall.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorChatChangeSoundConfig.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorChatEnd.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorChatReady.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorPause.proto",
            "src/protos/zt.live.interactive/CommonStateSignalAuthorResume.proto",
            "src/protos/zt.live.interactive/CommonStateSignalChatAccept.proto",
            "src/protos/zt.live.interactive/CommonStateSignalChatCall.proto",
            "src/protos/zt.live.interactive/CommonStateSignalChatEnd.proto",
            "src/protos/zt.live.interactive/CommonStateSignalChatReady.proto",
            "src/protos/zt.live.interactive/CommonStateSignalCurrentRedpackList.proto",
            "src/protos/zt.live.interactive/CommonStateSignalDisplayInfo.proto",
            "src/protos/zt.live.interactive/CommonStateSignalFeatureStateSync.proto",
            "src/protos/zt.live.interactive/CommonStateSignalLiveState.proto",
            "src/protos/zt.live.interactive/CommonStateSignalNewApplyUser.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPKAccept.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPkEnd.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPKInvitation.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPKReady.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPKSoundConfigChanged.proto",
            "src/protos/zt.live.interactive/CommonStateSignalPkStatistic.proto",
            "src/protos/zt.live.interactive/CommonStateSignalRecentComment.proto",
            "src/protos/zt.live.interactive/CommonStateSignalShoppingCart.proto",
            "src/protos/zt.live.interactive/CommonStateSignalTopUsers.proto",
            "src/protos/zt.live.interactive/CommonStateSignalWidget.proto",
            "src/protos/zt.live.interactive/CommonStateSignalWishSheetCurrentState.proto",
            "src/protos/zt.live.interactive/CsAckErrorCode.proto",
            "src/protos/zt.live.interactive/ImageCdnNode.proto",
            "src/protos/zt.live.interactive/KwaiStateSignalEcommerceCart.proto",
            "src/protos/zt.live.interactive/KwaiStateSignalEcommerceCartItemPopup.proto",
            "src/protos/zt.live.interactive/LiveFeatureState.proto",
            "src/protos/zt.live.interactive/PkAudienceContributionDetail.proto",
            "src/protos/zt.live.interactive/PkAudienceContributionInfo.proto",
            "src/protos/zt.live.interactive/PkPlayerInfo.proto",
            "src/protos/zt.live.interactive/PkPlayerRoundStatistic.proto",
            "src/protos/zt.live.interactive/PkPlayerStatistic.proto",
            "src/protos/zt.live.interactive/PkRoundInfo.proto",
            "src/protos/zt.live.interactive/TopBannerNotice.proto",
            "src/protos/zt.live.interactive/WidgetDisplayStyle.proto",
            "src/protos/zt.live.interactive/WidgetItem.proto",
            "src/protos/zt.live.interactive/WidgetPictureInfo.proto",
            "src/protos/zt.live.interactive/ZtDrawGiftInfo.proto",
            "src/protos/zt.live.interactive/ZtLiveActionSignalItem.proto",
            "src/protos/zt.live.interactive/ZtLiveCommonModelProto.proto",
            "src/protos/zt.live.interactive/ZtLiveCsCmd.proto",
            "src/protos/zt.live.interactive/ZtLiveCsEnterRoom.proto",
            "src/protos/zt.live.interactive/ZtLiveCsHeartbeat.proto",
            "src/protos/zt.live.interactive/ZtLiveCsUserExit.proto",
            "src/protos/zt.live.interactive/ZtLiveDownstreamPayloadErrorCode.proto",
            "src/protos/zt.live.interactive/ZtLiveNotifySignalItem.proto",
            "src/protos/zt.live.interactive/ZtLivePkProto.proto",
            "src/protos/zt.live.interactive/ZtLiveScActionSignal.proto",
            "src/protos/zt.live.interactive/ZtLiveScMessage.proto",
            "src/protos/zt.live.interactive/ZtLiveScNotifySignal.proto",
            "src/protos/zt.live.interactive/ZtLiveScStateSignal.proto",
            "src/protos/zt.live.interactive/ZtLiveScStatusChanged.proto",
            "src/protos/zt.live.interactive/ZtLiveScTicketInvalid.proto",
            "src/protos/zt.live.interactive/ZtLiveStartPlaySourceTypeProto.proto",
            "src/protos/zt.live.interactive/ZtLiveStateSignalItem.proto",
            "src/protos/zt.live.interactive/ZtLiveUserIdentity.proto",
            "src/protos/zt.live.interactive/ZtLiveUserInfo.proto",
            "src/protos/zt.live.interactive/ZtLiveWidgetProto.proto",
        ])
        .cargo_out_dir("protos")
        .run_from_script();

    tauri_build::build()
}
