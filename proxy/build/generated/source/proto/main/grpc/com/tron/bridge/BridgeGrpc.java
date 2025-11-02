package com.tron.bridge;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.62.2)",
    comments = "Source: bridge.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class BridgeGrpc {

  private BridgeGrpc() {}

  public static final java.lang.String SERVICE_NAME = "bridge.Bridge";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerJoinRequest,
      com.tron.bridge.Session.PlayerJoinResponse> getPlayerJoinMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerJoin",
      requestType = com.tron.bridge.Session.PlayerJoinRequest.class,
      responseType = com.tron.bridge.Session.PlayerJoinResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerJoinRequest,
      com.tron.bridge.Session.PlayerJoinResponse> getPlayerJoinMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerJoinRequest, com.tron.bridge.Session.PlayerJoinResponse> getPlayerJoinMethod;
    if ((getPlayerJoinMethod = BridgeGrpc.getPlayerJoinMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerJoinMethod = BridgeGrpc.getPlayerJoinMethod) == null) {
          BridgeGrpc.getPlayerJoinMethod = getPlayerJoinMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Session.PlayerJoinRequest, com.tron.bridge.Session.PlayerJoinResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerJoin"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Session.PlayerJoinRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Session.PlayerJoinResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerJoin"))
              .build();
        }
      }
    }
    return getPlayerJoinMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerLeaveRequest,
      com.tron.bridge.Session.PlayerLeaveResponse> getPlayerLeaveMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerLeave",
      requestType = com.tron.bridge.Session.PlayerLeaveRequest.class,
      responseType = com.tron.bridge.Session.PlayerLeaveResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerLeaveRequest,
      com.tron.bridge.Session.PlayerLeaveResponse> getPlayerLeaveMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Session.PlayerLeaveRequest, com.tron.bridge.Session.PlayerLeaveResponse> getPlayerLeaveMethod;
    if ((getPlayerLeaveMethod = BridgeGrpc.getPlayerLeaveMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerLeaveMethod = BridgeGrpc.getPlayerLeaveMethod) == null) {
          BridgeGrpc.getPlayerLeaveMethod = getPlayerLeaveMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Session.PlayerLeaveRequest, com.tron.bridge.Session.PlayerLeaveResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerLeave"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Session.PlayerLeaveRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Session.PlayerLeaveResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerLeave"))
              .build();
        }
      }
    }
    return getPlayerLeaveMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Balance.GetBalanceRequest,
      com.tron.bridge.Balance.GetBalanceResponse> getGetBalanceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetBalance",
      requestType = com.tron.bridge.Balance.GetBalanceRequest.class,
      responseType = com.tron.bridge.Balance.GetBalanceResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Balance.GetBalanceRequest,
      com.tron.bridge.Balance.GetBalanceResponse> getGetBalanceMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Balance.GetBalanceRequest, com.tron.bridge.Balance.GetBalanceResponse> getGetBalanceMethod;
    if ((getGetBalanceMethod = BridgeGrpc.getGetBalanceMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetBalanceMethod = BridgeGrpc.getGetBalanceMethod) == null) {
          BridgeGrpc.getGetBalanceMethod = getGetBalanceMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Balance.GetBalanceRequest, com.tron.bridge.Balance.GetBalanceResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetBalance"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Balance.GetBalanceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Balance.GetBalanceResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetBalance"))
              .build();
        }
      }
    }
    return getGetBalanceMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Balance.TransferBalanceRequest,
      com.tron.bridge.Balance.TransferBalanceResponse> getTransferBalanceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "TransferBalance",
      requestType = com.tron.bridge.Balance.TransferBalanceRequest.class,
      responseType = com.tron.bridge.Balance.TransferBalanceResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Balance.TransferBalanceRequest,
      com.tron.bridge.Balance.TransferBalanceResponse> getTransferBalanceMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Balance.TransferBalanceRequest, com.tron.bridge.Balance.TransferBalanceResponse> getTransferBalanceMethod;
    if ((getTransferBalanceMethod = BridgeGrpc.getTransferBalanceMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getTransferBalanceMethod = BridgeGrpc.getTransferBalanceMethod) == null) {
          BridgeGrpc.getTransferBalanceMethod = getTransferBalanceMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Balance.TransferBalanceRequest, com.tron.bridge.Balance.TransferBalanceResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "TransferBalance"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Balance.TransferBalanceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Balance.TransferBalanceResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("TransferBalance"))
              .build();
        }
      }
    }
    return getTransferBalanceMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.OverallLeaderboardRequest,
      com.tron.bridge.Leaderboard.OverallLeaderboardResponse> getOverallLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "OverallLeaderboard",
      requestType = com.tron.bridge.Leaderboard.OverallLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.OverallLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.OverallLeaderboardRequest,
      com.tron.bridge.Leaderboard.OverallLeaderboardResponse> getOverallLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.OverallLeaderboardRequest, com.tron.bridge.Leaderboard.OverallLeaderboardResponse> getOverallLeaderboardMethod;
    if ((getOverallLeaderboardMethod = BridgeGrpc.getOverallLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getOverallLeaderboardMethod = BridgeGrpc.getOverallLeaderboardMethod) == null) {
          BridgeGrpc.getOverallLeaderboardMethod = getOverallLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.OverallLeaderboardRequest, com.tron.bridge.Leaderboard.OverallLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "OverallLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.OverallLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.OverallLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("OverallLeaderboard"))
              .build();
        }
      }
    }
    return getOverallLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.CoinsLeaderboardRequest,
      com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> getCoinsLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CoinsLeaderboard",
      requestType = com.tron.bridge.Leaderboard.CoinsLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.CoinsLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.CoinsLeaderboardRequest,
      com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> getCoinsLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.CoinsLeaderboardRequest, com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> getCoinsLeaderboardMethod;
    if ((getCoinsLeaderboardMethod = BridgeGrpc.getCoinsLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getCoinsLeaderboardMethod = BridgeGrpc.getCoinsLeaderboardMethod) == null) {
          BridgeGrpc.getCoinsLeaderboardMethod = getCoinsLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.CoinsLeaderboardRequest, com.tron.bridge.Leaderboard.CoinsLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CoinsLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.CoinsLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.CoinsLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("CoinsLeaderboard"))
              .build();
        }
      }
    }
    return getCoinsLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.TeamsLeaderboardRequest,
      com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> getTeamsLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "TeamsLeaderboard",
      requestType = com.tron.bridge.Leaderboard.TeamsLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.TeamsLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.TeamsLeaderboardRequest,
      com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> getTeamsLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.TeamsLeaderboardRequest, com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> getTeamsLeaderboardMethod;
    if ((getTeamsLeaderboardMethod = BridgeGrpc.getTeamsLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getTeamsLeaderboardMethod = BridgeGrpc.getTeamsLeaderboardMethod) == null) {
          BridgeGrpc.getTeamsLeaderboardMethod = getTeamsLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.TeamsLeaderboardRequest, com.tron.bridge.Leaderboard.TeamsLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "TeamsLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.TeamsLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.TeamsLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("TeamsLeaderboard"))
              .build();
        }
      }
    }
    return getTeamsLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KdaLeaderboardRequest,
      com.tron.bridge.Leaderboard.KdaLeaderboardResponse> getKdaLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "KdaLeaderboard",
      requestType = com.tron.bridge.Leaderboard.KdaLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.KdaLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KdaLeaderboardRequest,
      com.tron.bridge.Leaderboard.KdaLeaderboardResponse> getKdaLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KdaLeaderboardRequest, com.tron.bridge.Leaderboard.KdaLeaderboardResponse> getKdaLeaderboardMethod;
    if ((getKdaLeaderboardMethod = BridgeGrpc.getKdaLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getKdaLeaderboardMethod = BridgeGrpc.getKdaLeaderboardMethod) == null) {
          BridgeGrpc.getKdaLeaderboardMethod = getKdaLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.KdaLeaderboardRequest, com.tron.bridge.Leaderboard.KdaLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "KdaLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.KdaLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.KdaLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("KdaLeaderboard"))
              .build();
        }
      }
    }
    return getKdaLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.DeathsLeaderboardRequest,
      com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> getDeathsLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeathsLeaderboard",
      requestType = com.tron.bridge.Leaderboard.DeathsLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.DeathsLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.DeathsLeaderboardRequest,
      com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> getDeathsLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.DeathsLeaderboardRequest, com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> getDeathsLeaderboardMethod;
    if ((getDeathsLeaderboardMethod = BridgeGrpc.getDeathsLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getDeathsLeaderboardMethod = BridgeGrpc.getDeathsLeaderboardMethod) == null) {
          BridgeGrpc.getDeathsLeaderboardMethod = getDeathsLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.DeathsLeaderboardRequest, com.tron.bridge.Leaderboard.DeathsLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeathsLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.DeathsLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.DeathsLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("DeathsLeaderboard"))
              .build();
        }
      }
    }
    return getDeathsLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KillsLeaderboardRequest,
      com.tron.bridge.Leaderboard.KillsLeaderboardResponse> getKillsLeaderboardMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "KillsLeaderboard",
      requestType = com.tron.bridge.Leaderboard.KillsLeaderboardRequest.class,
      responseType = com.tron.bridge.Leaderboard.KillsLeaderboardResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KillsLeaderboardRequest,
      com.tron.bridge.Leaderboard.KillsLeaderboardResponse> getKillsLeaderboardMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Leaderboard.KillsLeaderboardRequest, com.tron.bridge.Leaderboard.KillsLeaderboardResponse> getKillsLeaderboardMethod;
    if ((getKillsLeaderboardMethod = BridgeGrpc.getKillsLeaderboardMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getKillsLeaderboardMethod = BridgeGrpc.getKillsLeaderboardMethod) == null) {
          BridgeGrpc.getKillsLeaderboardMethod = getKillsLeaderboardMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Leaderboard.KillsLeaderboardRequest, com.tron.bridge.Leaderboard.KillsLeaderboardResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "KillsLeaderboard"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.KillsLeaderboardRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Leaderboard.KillsLeaderboardResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("KillsLeaderboard"))
              .build();
        }
      }
    }
    return getKillsLeaderboardMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendsRequest,
      com.tron.bridge.Friends.GetFriendsResponse> getGetFriendsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetFriends",
      requestType = com.tron.bridge.Friends.GetFriendsRequest.class,
      responseType = com.tron.bridge.Friends.GetFriendsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendsRequest,
      com.tron.bridge.Friends.GetFriendsResponse> getGetFriendsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendsRequest, com.tron.bridge.Friends.GetFriendsResponse> getGetFriendsMethod;
    if ((getGetFriendsMethod = BridgeGrpc.getGetFriendsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetFriendsMethod = BridgeGrpc.getGetFriendsMethod) == null) {
          BridgeGrpc.getGetFriendsMethod = getGetFriendsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.GetFriendsRequest, com.tron.bridge.Friends.GetFriendsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetFriends"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.GetFriendsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.GetFriendsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetFriends"))
              .build();
        }
      }
    }
    return getGetFriendsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendsRequest,
      com.tron.bridge.Friends.ListFriendsResponse> getListFriendsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListFriends",
      requestType = com.tron.bridge.Friends.ListFriendsRequest.class,
      responseType = com.tron.bridge.Friends.ListFriendsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendsRequest,
      com.tron.bridge.Friends.ListFriendsResponse> getListFriendsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendsRequest, com.tron.bridge.Friends.ListFriendsResponse> getListFriendsMethod;
    if ((getListFriendsMethod = BridgeGrpc.getListFriendsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getListFriendsMethod = BridgeGrpc.getListFriendsMethod) == null) {
          BridgeGrpc.getListFriendsMethod = getListFriendsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.ListFriendsRequest, com.tron.bridge.Friends.ListFriendsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListFriends"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.ListFriendsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.ListFriendsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ListFriends"))
              .build();
        }
      }
    }
    return getListFriendsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.SendFriendRequestRequest,
      com.tron.bridge.Friends.SendFriendRequestResponse> getSendFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SendFriendRequest",
      requestType = com.tron.bridge.Friends.SendFriendRequestRequest.class,
      responseType = com.tron.bridge.Friends.SendFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.SendFriendRequestRequest,
      com.tron.bridge.Friends.SendFriendRequestResponse> getSendFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.SendFriendRequestRequest, com.tron.bridge.Friends.SendFriendRequestResponse> getSendFriendRequestMethod;
    if ((getSendFriendRequestMethod = BridgeGrpc.getSendFriendRequestMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSendFriendRequestMethod = BridgeGrpc.getSendFriendRequestMethod) == null) {
          BridgeGrpc.getSendFriendRequestMethod = getSendFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.SendFriendRequestRequest, com.tron.bridge.Friends.SendFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SendFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.SendFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.SendFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SendFriendRequest"))
              .build();
        }
      }
    }
    return getSendFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.AcceptFriendRequestRequest,
      com.tron.bridge.Friends.AcceptFriendRequestResponse> getAcceptFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AcceptFriendRequest",
      requestType = com.tron.bridge.Friends.AcceptFriendRequestRequest.class,
      responseType = com.tron.bridge.Friends.AcceptFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.AcceptFriendRequestRequest,
      com.tron.bridge.Friends.AcceptFriendRequestResponse> getAcceptFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.AcceptFriendRequestRequest, com.tron.bridge.Friends.AcceptFriendRequestResponse> getAcceptFriendRequestMethod;
    if ((getAcceptFriendRequestMethod = BridgeGrpc.getAcceptFriendRequestMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getAcceptFriendRequestMethod = BridgeGrpc.getAcceptFriendRequestMethod) == null) {
          BridgeGrpc.getAcceptFriendRequestMethod = getAcceptFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.AcceptFriendRequestRequest, com.tron.bridge.Friends.AcceptFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AcceptFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.AcceptFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.AcceptFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("AcceptFriendRequest"))
              .build();
        }
      }
    }
    return getAcceptFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.RejectFriendRequestRequest,
      com.tron.bridge.Friends.RejectFriendRequestResponse> getRejectFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RejectFriendRequest",
      requestType = com.tron.bridge.Friends.RejectFriendRequestRequest.class,
      responseType = com.tron.bridge.Friends.RejectFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.RejectFriendRequestRequest,
      com.tron.bridge.Friends.RejectFriendRequestResponse> getRejectFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.RejectFriendRequestRequest, com.tron.bridge.Friends.RejectFriendRequestResponse> getRejectFriendRequestMethod;
    if ((getRejectFriendRequestMethod = BridgeGrpc.getRejectFriendRequestMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getRejectFriendRequestMethod = BridgeGrpc.getRejectFriendRequestMethod) == null) {
          BridgeGrpc.getRejectFriendRequestMethod = getRejectFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.RejectFriendRequestRequest, com.tron.bridge.Friends.RejectFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RejectFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RejectFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RejectFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("RejectFriendRequest"))
              .build();
        }
      }
    }
    return getRejectFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendRequestsRequest,
      com.tron.bridge.Friends.GetFriendRequestsResponse> getGetFriendRequestsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetFriendRequests",
      requestType = com.tron.bridge.Friends.GetFriendRequestsRequest.class,
      responseType = com.tron.bridge.Friends.GetFriendRequestsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendRequestsRequest,
      com.tron.bridge.Friends.GetFriendRequestsResponse> getGetFriendRequestsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.GetFriendRequestsRequest, com.tron.bridge.Friends.GetFriendRequestsResponse> getGetFriendRequestsMethod;
    if ((getGetFriendRequestsMethod = BridgeGrpc.getGetFriendRequestsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetFriendRequestsMethod = BridgeGrpc.getGetFriendRequestsMethod) == null) {
          BridgeGrpc.getGetFriendRequestsMethod = getGetFriendRequestsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.GetFriendRequestsRequest, com.tron.bridge.Friends.GetFriendRequestsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetFriendRequests"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.GetFriendRequestsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.GetFriendRequestsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetFriendRequests"))
              .build();
        }
      }
    }
    return getGetFriendRequestsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendRequestsRequest,
      com.tron.bridge.Friends.ListFriendRequestsResponse> getListFriendRequestsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListFriendRequests",
      requestType = com.tron.bridge.Friends.ListFriendRequestsRequest.class,
      responseType = com.tron.bridge.Friends.ListFriendRequestsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendRequestsRequest,
      com.tron.bridge.Friends.ListFriendRequestsResponse> getListFriendRequestsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.ListFriendRequestsRequest, com.tron.bridge.Friends.ListFriendRequestsResponse> getListFriendRequestsMethod;
    if ((getListFriendRequestsMethod = BridgeGrpc.getListFriendRequestsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getListFriendRequestsMethod = BridgeGrpc.getListFriendRequestsMethod) == null) {
          BridgeGrpc.getListFriendRequestsMethod = getListFriendRequestsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.ListFriendRequestsRequest, com.tron.bridge.Friends.ListFriendRequestsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListFriendRequests"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.ListFriendRequestsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.ListFriendRequestsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ListFriendRequests"))
              .build();
        }
      }
    }
    return getListFriendRequestsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveFriendRequest,
      com.tron.bridge.Friends.RemoveFriendResponse> getRemoveFriendMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveFriend",
      requestType = com.tron.bridge.Friends.RemoveFriendRequest.class,
      responseType = com.tron.bridge.Friends.RemoveFriendResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveFriendRequest,
      com.tron.bridge.Friends.RemoveFriendResponse> getRemoveFriendMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveFriendRequest, com.tron.bridge.Friends.RemoveFriendResponse> getRemoveFriendMethod;
    if ((getRemoveFriendMethod = BridgeGrpc.getRemoveFriendMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getRemoveFriendMethod = BridgeGrpc.getRemoveFriendMethod) == null) {
          BridgeGrpc.getRemoveFriendMethod = getRemoveFriendMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.RemoveFriendRequest, com.tron.bridge.Friends.RemoveFriendResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveFriend"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RemoveFriendRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RemoveFriendResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("RemoveFriend"))
              .build();
        }
      }
    }
    return getRemoveFriendMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.CreateTeamRequest,
      com.tron.bridge.Teams.CreateTeamResponse> getCreateTeamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateTeam",
      requestType = com.tron.bridge.Teams.CreateTeamRequest.class,
      responseType = com.tron.bridge.Teams.CreateTeamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.CreateTeamRequest,
      com.tron.bridge.Teams.CreateTeamResponse> getCreateTeamMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.CreateTeamRequest, com.tron.bridge.Teams.CreateTeamResponse> getCreateTeamMethod;
    if ((getCreateTeamMethod = BridgeGrpc.getCreateTeamMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getCreateTeamMethod = BridgeGrpc.getCreateTeamMethod) == null) {
          BridgeGrpc.getCreateTeamMethod = getCreateTeamMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.CreateTeamRequest, com.tron.bridge.Teams.CreateTeamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateTeam"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.CreateTeamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.CreateTeamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("CreateTeam"))
              .build();
        }
      }
    }
    return getCreateTeamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.DeleteTeamRequest,
      com.tron.bridge.Teams.DeleteTeamResponse> getDeleteTeamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteTeam",
      requestType = com.tron.bridge.Teams.DeleteTeamRequest.class,
      responseType = com.tron.bridge.Teams.DeleteTeamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.DeleteTeamRequest,
      com.tron.bridge.Teams.DeleteTeamResponse> getDeleteTeamMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.DeleteTeamRequest, com.tron.bridge.Teams.DeleteTeamResponse> getDeleteTeamMethod;
    if ((getDeleteTeamMethod = BridgeGrpc.getDeleteTeamMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getDeleteTeamMethod = BridgeGrpc.getDeleteTeamMethod) == null) {
          BridgeGrpc.getDeleteTeamMethod = getDeleteTeamMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.DeleteTeamRequest, com.tron.bridge.Teams.DeleteTeamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteTeam"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.DeleteTeamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.DeleteTeamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("DeleteTeam"))
              .build();
        }
      }
    }
    return getDeleteTeamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.LeaveTeamRequest,
      com.tron.bridge.Teams.LeaveTeamResponse> getLeaveTeamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LeaveTeam",
      requestType = com.tron.bridge.Teams.LeaveTeamRequest.class,
      responseType = com.tron.bridge.Teams.LeaveTeamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.LeaveTeamRequest,
      com.tron.bridge.Teams.LeaveTeamResponse> getLeaveTeamMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.LeaveTeamRequest, com.tron.bridge.Teams.LeaveTeamResponse> getLeaveTeamMethod;
    if ((getLeaveTeamMethod = BridgeGrpc.getLeaveTeamMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getLeaveTeamMethod = BridgeGrpc.getLeaveTeamMethod) == null) {
          BridgeGrpc.getLeaveTeamMethod = getLeaveTeamMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.LeaveTeamRequest, com.tron.bridge.Teams.LeaveTeamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LeaveTeam"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.LeaveTeamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.LeaveTeamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("LeaveTeam"))
              .build();
        }
      }
    }
    return getLeaveTeamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.JoinTeamRequest,
      com.tron.bridge.Teams.JoinTeamResponse> getJoinTeamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "JoinTeam",
      requestType = com.tron.bridge.Teams.JoinTeamRequest.class,
      responseType = com.tron.bridge.Teams.JoinTeamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.JoinTeamRequest,
      com.tron.bridge.Teams.JoinTeamResponse> getJoinTeamMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.JoinTeamRequest, com.tron.bridge.Teams.JoinTeamResponse> getJoinTeamMethod;
    if ((getJoinTeamMethod = BridgeGrpc.getJoinTeamMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getJoinTeamMethod = BridgeGrpc.getJoinTeamMethod) == null) {
          BridgeGrpc.getJoinTeamMethod = getJoinTeamMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.JoinTeamRequest, com.tron.bridge.Teams.JoinTeamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "JoinTeam"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.JoinTeamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.JoinTeamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("JoinTeam"))
              .build();
        }
      }
    }
    return getJoinTeamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.SendTeamInviteRequest,
      com.tron.bridge.Teams.SendTeamInviteResponse> getSendTeamInviteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SendTeamInvite",
      requestType = com.tron.bridge.Teams.SendTeamInviteRequest.class,
      responseType = com.tron.bridge.Teams.SendTeamInviteResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.SendTeamInviteRequest,
      com.tron.bridge.Teams.SendTeamInviteResponse> getSendTeamInviteMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.SendTeamInviteRequest, com.tron.bridge.Teams.SendTeamInviteResponse> getSendTeamInviteMethod;
    if ((getSendTeamInviteMethod = BridgeGrpc.getSendTeamInviteMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSendTeamInviteMethod = BridgeGrpc.getSendTeamInviteMethod) == null) {
          BridgeGrpc.getSendTeamInviteMethod = getSendTeamInviteMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.SendTeamInviteRequest, com.tron.bridge.Teams.SendTeamInviteResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SendTeamInvite"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.SendTeamInviteRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.SendTeamInviteResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SendTeamInvite"))
              .build();
        }
      }
    }
    return getSendTeamInviteMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.AcceptTeamInviteRequest,
      com.tron.bridge.Teams.AcceptTeamInviteResponse> getAcceptTeamInviteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AcceptTeamInvite",
      requestType = com.tron.bridge.Teams.AcceptTeamInviteRequest.class,
      responseType = com.tron.bridge.Teams.AcceptTeamInviteResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.AcceptTeamInviteRequest,
      com.tron.bridge.Teams.AcceptTeamInviteResponse> getAcceptTeamInviteMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.AcceptTeamInviteRequest, com.tron.bridge.Teams.AcceptTeamInviteResponse> getAcceptTeamInviteMethod;
    if ((getAcceptTeamInviteMethod = BridgeGrpc.getAcceptTeamInviteMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getAcceptTeamInviteMethod = BridgeGrpc.getAcceptTeamInviteMethod) == null) {
          BridgeGrpc.getAcceptTeamInviteMethod = getAcceptTeamInviteMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.AcceptTeamInviteRequest, com.tron.bridge.Teams.AcceptTeamInviteResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AcceptTeamInvite"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.AcceptTeamInviteRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.AcceptTeamInviteResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("AcceptTeamInvite"))
              .build();
        }
      }
    }
    return getAcceptTeamInviteMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.RejectTeamInviteRequest,
      com.tron.bridge.Teams.RejectTeamInviteResponse> getRejectTeamInviteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RejectTeamInvite",
      requestType = com.tron.bridge.Teams.RejectTeamInviteRequest.class,
      responseType = com.tron.bridge.Teams.RejectTeamInviteResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.RejectTeamInviteRequest,
      com.tron.bridge.Teams.RejectTeamInviteResponse> getRejectTeamInviteMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.RejectTeamInviteRequest, com.tron.bridge.Teams.RejectTeamInviteResponse> getRejectTeamInviteMethod;
    if ((getRejectTeamInviteMethod = BridgeGrpc.getRejectTeamInviteMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getRejectTeamInviteMethod = BridgeGrpc.getRejectTeamInviteMethod) == null) {
          BridgeGrpc.getRejectTeamInviteMethod = getRejectTeamInviteMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.RejectTeamInviteRequest, com.tron.bridge.Teams.RejectTeamInviteResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RejectTeamInvite"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.RejectTeamInviteRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.RejectTeamInviteResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("RejectTeamInvite"))
              .build();
        }
      }
    }
    return getRejectTeamInviteMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetTeamMembersRequest,
      com.tron.bridge.Teams.GetTeamMembersResponse> getGetTeamMembersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetTeamMembers",
      requestType = com.tron.bridge.Teams.GetTeamMembersRequest.class,
      responseType = com.tron.bridge.Teams.GetTeamMembersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetTeamMembersRequest,
      com.tron.bridge.Teams.GetTeamMembersResponse> getGetTeamMembersMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetTeamMembersRequest, com.tron.bridge.Teams.GetTeamMembersResponse> getGetTeamMembersMethod;
    if ((getGetTeamMembersMethod = BridgeGrpc.getGetTeamMembersMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetTeamMembersMethod = BridgeGrpc.getGetTeamMembersMethod) == null) {
          BridgeGrpc.getGetTeamMembersMethod = getGetTeamMembersMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.GetTeamMembersRequest, com.tron.bridge.Teams.GetTeamMembersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetTeamMembers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.GetTeamMembersRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.GetTeamMembersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetTeamMembers"))
              .build();
        }
      }
    }
    return getGetTeamMembersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveTeamMemberRequest,
      com.tron.bridge.Friends.RemoveTeamMemberResponse> getRemoveTeamMemberMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveTeamMember",
      requestType = com.tron.bridge.Friends.RemoveTeamMemberRequest.class,
      responseType = com.tron.bridge.Friends.RemoveTeamMemberResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveTeamMemberRequest,
      com.tron.bridge.Friends.RemoveTeamMemberResponse> getRemoveTeamMemberMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Friends.RemoveTeamMemberRequest, com.tron.bridge.Friends.RemoveTeamMemberResponse> getRemoveTeamMemberMethod;
    if ((getRemoveTeamMemberMethod = BridgeGrpc.getRemoveTeamMemberMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getRemoveTeamMemberMethod = BridgeGrpc.getRemoveTeamMemberMethod) == null) {
          BridgeGrpc.getRemoveTeamMemberMethod = getRemoveTeamMemberMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Friends.RemoveTeamMemberRequest, com.tron.bridge.Friends.RemoveTeamMemberResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveTeamMember"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RemoveTeamMemberRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Friends.RemoveTeamMemberResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("RemoveTeamMember"))
              .build();
        }
      }
    }
    return getRemoveTeamMemberMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.PromoteTeamMemberRequest,
      com.tron.bridge.Teams.PromoteTeamMemberResponse> getPromoteTeamMemberMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PromoteTeamMember",
      requestType = com.tron.bridge.Teams.PromoteTeamMemberRequest.class,
      responseType = com.tron.bridge.Teams.PromoteTeamMemberResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.PromoteTeamMemberRequest,
      com.tron.bridge.Teams.PromoteTeamMemberResponse> getPromoteTeamMemberMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.PromoteTeamMemberRequest, com.tron.bridge.Teams.PromoteTeamMemberResponse> getPromoteTeamMemberMethod;
    if ((getPromoteTeamMemberMethod = BridgeGrpc.getPromoteTeamMemberMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPromoteTeamMemberMethod = BridgeGrpc.getPromoteTeamMemberMethod) == null) {
          BridgeGrpc.getPromoteTeamMemberMethod = getPromoteTeamMemberMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.PromoteTeamMemberRequest, com.tron.bridge.Teams.PromoteTeamMemberResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PromoteTeamMember"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.PromoteTeamMemberRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.PromoteTeamMemberResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PromoteTeamMember"))
              .build();
        }
      }
    }
    return getPromoteTeamMemberMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetOpenTeamsRequest,
      com.tron.bridge.Teams.GetOpenTeamsResponse> getGetOpenTeamsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetOpenTeams",
      requestType = com.tron.bridge.Teams.GetOpenTeamsRequest.class,
      responseType = com.tron.bridge.Teams.GetOpenTeamsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetOpenTeamsRequest,
      com.tron.bridge.Teams.GetOpenTeamsResponse> getGetOpenTeamsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Teams.GetOpenTeamsRequest, com.tron.bridge.Teams.GetOpenTeamsResponse> getGetOpenTeamsMethod;
    if ((getGetOpenTeamsMethod = BridgeGrpc.getGetOpenTeamsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetOpenTeamsMethod = BridgeGrpc.getGetOpenTeamsMethod) == null) {
          BridgeGrpc.getGetOpenTeamsMethod = getGetOpenTeamsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Teams.GetOpenTeamsRequest, com.tron.bridge.Teams.GetOpenTeamsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetOpenTeams"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.GetOpenTeamsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Teams.GetOpenTeamsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetOpenTeams"))
              .build();
        }
      }
    }
    return getGetOpenTeamsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Shop.BuyItemRequest,
      com.tron.bridge.Shop.BuyItemResponse> getBuyItemMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BuyItem",
      requestType = com.tron.bridge.Shop.BuyItemRequest.class,
      responseType = com.tron.bridge.Shop.BuyItemResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Shop.BuyItemRequest,
      com.tron.bridge.Shop.BuyItemResponse> getBuyItemMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Shop.BuyItemRequest, com.tron.bridge.Shop.BuyItemResponse> getBuyItemMethod;
    if ((getBuyItemMethod = BridgeGrpc.getBuyItemMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getBuyItemMethod = BridgeGrpc.getBuyItemMethod) == null) {
          BridgeGrpc.getBuyItemMethod = getBuyItemMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Shop.BuyItemRequest, com.tron.bridge.Shop.BuyItemResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BuyItem"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.BuyItemRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.BuyItemResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("BuyItem"))
              .build();
        }
      }
    }
    return getBuyItemMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Shop.SellItemRequest,
      com.tron.bridge.Shop.SellItemResponse> getSellItemMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SellItem",
      requestType = com.tron.bridge.Shop.SellItemRequest.class,
      responseType = com.tron.bridge.Shop.SellItemResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Shop.SellItemRequest,
      com.tron.bridge.Shop.SellItemResponse> getSellItemMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Shop.SellItemRequest, com.tron.bridge.Shop.SellItemResponse> getSellItemMethod;
    if ((getSellItemMethod = BridgeGrpc.getSellItemMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSellItemMethod = BridgeGrpc.getSellItemMethod) == null) {
          BridgeGrpc.getSellItemMethod = getSellItemMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Shop.SellItemRequest, com.tron.bridge.Shop.SellItemResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SellItem"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.SellItemRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.SellItemResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SellItem"))
              .build();
        }
      }
    }
    return getSellItemMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Shop.GetItemsRequest,
      com.tron.bridge.Shop.GetItemsResponse> getGetItemsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetItems",
      requestType = com.tron.bridge.Shop.GetItemsRequest.class,
      responseType = com.tron.bridge.Shop.GetItemsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Shop.GetItemsRequest,
      com.tron.bridge.Shop.GetItemsResponse> getGetItemsMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Shop.GetItemsRequest, com.tron.bridge.Shop.GetItemsResponse> getGetItemsMethod;
    if ((getGetItemsMethod = BridgeGrpc.getGetItemsMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetItemsMethod = BridgeGrpc.getGetItemsMethod) == null) {
          BridgeGrpc.getGetItemsMethod = getGetItemsMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Shop.GetItemsRequest, com.tron.bridge.Shop.GetItemsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetItems"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.GetItemsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Shop.GetItemsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetItems"))
              .build();
        }
      }
    }
    return getGetItemsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.BuyPrefixRequest,
      com.tron.bridge.Prefix.BuyPrefixResponse> getBuyPrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BuyPrefix",
      requestType = com.tron.bridge.Prefix.BuyPrefixRequest.class,
      responseType = com.tron.bridge.Prefix.BuyPrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.BuyPrefixRequest,
      com.tron.bridge.Prefix.BuyPrefixResponse> getBuyPrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.BuyPrefixRequest, com.tron.bridge.Prefix.BuyPrefixResponse> getBuyPrefixMethod;
    if ((getBuyPrefixMethod = BridgeGrpc.getBuyPrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getBuyPrefixMethod = BridgeGrpc.getBuyPrefixMethod) == null) {
          BridgeGrpc.getBuyPrefixMethod = getBuyPrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.BuyPrefixRequest, com.tron.bridge.Prefix.BuyPrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BuyPrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.BuyPrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.BuyPrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("BuyPrefix"))
              .build();
        }
      }
    }
    return getBuyPrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.SelectPrefixRequest,
      com.tron.bridge.Prefix.SelectPrefixResponse> getSelectPrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SelectPrefix",
      requestType = com.tron.bridge.Prefix.SelectPrefixRequest.class,
      responseType = com.tron.bridge.Prefix.SelectPrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.SelectPrefixRequest,
      com.tron.bridge.Prefix.SelectPrefixResponse> getSelectPrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.SelectPrefixRequest, com.tron.bridge.Prefix.SelectPrefixResponse> getSelectPrefixMethod;
    if ((getSelectPrefixMethod = BridgeGrpc.getSelectPrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSelectPrefixMethod = BridgeGrpc.getSelectPrefixMethod) == null) {
          BridgeGrpc.getSelectPrefixMethod = getSelectPrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.SelectPrefixRequest, com.tron.bridge.Prefix.SelectPrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SelectPrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.SelectPrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.SelectPrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SelectPrefix"))
              .build();
        }
      }
    }
    return getSelectPrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetAllPrefixRequest,
      com.tron.bridge.Prefix.GetAllPrefixResponse> getGetAllPrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetAllPrefix",
      requestType = com.tron.bridge.Prefix.GetAllPrefixRequest.class,
      responseType = com.tron.bridge.Prefix.GetAllPrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetAllPrefixRequest,
      com.tron.bridge.Prefix.GetAllPrefixResponse> getGetAllPrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetAllPrefixRequest, com.tron.bridge.Prefix.GetAllPrefixResponse> getGetAllPrefixMethod;
    if ((getGetAllPrefixMethod = BridgeGrpc.getGetAllPrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetAllPrefixMethod = BridgeGrpc.getGetAllPrefixMethod) == null) {
          BridgeGrpc.getGetAllPrefixMethod = getGetAllPrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.GetAllPrefixRequest, com.tron.bridge.Prefix.GetAllPrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetAllPrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetAllPrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetAllPrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetAllPrefix"))
              .build();
        }
      }
    }
    return getGetAllPrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetOwnedPrefixRequest,
      com.tron.bridge.Prefix.GetOwnedPrefixResponse> getGetOwnedPrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetOwnedPrefix",
      requestType = com.tron.bridge.Prefix.GetOwnedPrefixRequest.class,
      responseType = com.tron.bridge.Prefix.GetOwnedPrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetOwnedPrefixRequest,
      com.tron.bridge.Prefix.GetOwnedPrefixResponse> getGetOwnedPrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetOwnedPrefixRequest, com.tron.bridge.Prefix.GetOwnedPrefixResponse> getGetOwnedPrefixMethod;
    if ((getGetOwnedPrefixMethod = BridgeGrpc.getGetOwnedPrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetOwnedPrefixMethod = BridgeGrpc.getGetOwnedPrefixMethod) == null) {
          BridgeGrpc.getGetOwnedPrefixMethod = getGetOwnedPrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.GetOwnedPrefixRequest, com.tron.bridge.Prefix.GetOwnedPrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetOwnedPrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetOwnedPrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetOwnedPrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetOwnedPrefix"))
              .build();
        }
      }
    }
    return getGetOwnedPrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetCurrentPrefixRequest,
      com.tron.bridge.Prefix.GetCurrentPrefixResponse> getGetCurrentPrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "GetCurrentPrefix",
      requestType = com.tron.bridge.Prefix.GetCurrentPrefixRequest.class,
      responseType = com.tron.bridge.Prefix.GetCurrentPrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetCurrentPrefixRequest,
      com.tron.bridge.Prefix.GetCurrentPrefixResponse> getGetCurrentPrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.GetCurrentPrefixRequest, com.tron.bridge.Prefix.GetCurrentPrefixResponse> getGetCurrentPrefixMethod;
    if ((getGetCurrentPrefixMethod = BridgeGrpc.getGetCurrentPrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getGetCurrentPrefixMethod = BridgeGrpc.getGetCurrentPrefixMethod) == null) {
          BridgeGrpc.getGetCurrentPrefixMethod = getGetCurrentPrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.GetCurrentPrefixRequest, com.tron.bridge.Prefix.GetCurrentPrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "GetCurrentPrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetCurrentPrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.GetCurrentPrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("GetCurrentPrefix"))
              .build();
        }
      }
    }
    return getGetCurrentPrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.CreatePrefixRequest,
      com.tron.bridge.Prefix.CreatePrefixResponse> getCreatePrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreatePrefix",
      requestType = com.tron.bridge.Prefix.CreatePrefixRequest.class,
      responseType = com.tron.bridge.Prefix.CreatePrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.CreatePrefixRequest,
      com.tron.bridge.Prefix.CreatePrefixResponse> getCreatePrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.CreatePrefixRequest, com.tron.bridge.Prefix.CreatePrefixResponse> getCreatePrefixMethod;
    if ((getCreatePrefixMethod = BridgeGrpc.getCreatePrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getCreatePrefixMethod = BridgeGrpc.getCreatePrefixMethod) == null) {
          BridgeGrpc.getCreatePrefixMethod = getCreatePrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.CreatePrefixRequest, com.tron.bridge.Prefix.CreatePrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreatePrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.CreatePrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.CreatePrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("CreatePrefix"))
              .build();
        }
      }
    }
    return getCreatePrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Prefix.DeletePrefixRequest,
      com.tron.bridge.Prefix.DeletePrefixResponse> getDeletePrefixMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeletePrefix",
      requestType = com.tron.bridge.Prefix.DeletePrefixRequest.class,
      responseType = com.tron.bridge.Prefix.DeletePrefixResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Prefix.DeletePrefixRequest,
      com.tron.bridge.Prefix.DeletePrefixResponse> getDeletePrefixMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Prefix.DeletePrefixRequest, com.tron.bridge.Prefix.DeletePrefixResponse> getDeletePrefixMethod;
    if ((getDeletePrefixMethod = BridgeGrpc.getDeletePrefixMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getDeletePrefixMethod = BridgeGrpc.getDeletePrefixMethod) == null) {
          BridgeGrpc.getDeletePrefixMethod = getDeletePrefixMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Prefix.DeletePrefixRequest, com.tron.bridge.Prefix.DeletePrefixResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeletePrefix"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.DeletePrefixRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Prefix.DeletePrefixResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("DeletePrefix"))
              .build();
        }
      }
    }
    return getDeletePrefixMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerDeathRequest,
      com.tron.bridge.Player.PlayerDeathResponse> getPlayerDeathMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerDeath",
      requestType = com.tron.bridge.Player.PlayerDeathRequest.class,
      responseType = com.tron.bridge.Player.PlayerDeathResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerDeathRequest,
      com.tron.bridge.Player.PlayerDeathResponse> getPlayerDeathMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerDeathRequest, com.tron.bridge.Player.PlayerDeathResponse> getPlayerDeathMethod;
    if ((getPlayerDeathMethod = BridgeGrpc.getPlayerDeathMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerDeathMethod = BridgeGrpc.getPlayerDeathMethod) == null) {
          BridgeGrpc.getPlayerDeathMethod = getPlayerDeathMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Player.PlayerDeathRequest, com.tron.bridge.Player.PlayerDeathResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerDeath"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerDeathRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerDeathResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerDeath"))
              .build();
        }
      }
    }
    return getPlayerDeathMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerKillRequest,
      com.tron.bridge.Player.PlayerKillResponse> getPlayerKillMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerKill",
      requestType = com.tron.bridge.Player.PlayerKillRequest.class,
      responseType = com.tron.bridge.Player.PlayerKillResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerKillRequest,
      com.tron.bridge.Player.PlayerKillResponse> getPlayerKillMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerKillRequest, com.tron.bridge.Player.PlayerKillResponse> getPlayerKillMethod;
    if ((getPlayerKillMethod = BridgeGrpc.getPlayerKillMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerKillMethod = BridgeGrpc.getPlayerKillMethod) == null) {
          BridgeGrpc.getPlayerKillMethod = getPlayerKillMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Player.PlayerKillRequest, com.tron.bridge.Player.PlayerKillResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerKill"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerKillRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerKillResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerKill"))
              .build();
        }
      }
    }
    return getPlayerKillMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerPlaceBlockRequest,
      com.tron.bridge.Player.PlayerPlaceBlockResponse> getPlayerPlaceBlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerPlaceBlock",
      requestType = com.tron.bridge.Player.PlayerPlaceBlockRequest.class,
      responseType = com.tron.bridge.Player.PlayerPlaceBlockResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerPlaceBlockRequest,
      com.tron.bridge.Player.PlayerPlaceBlockResponse> getPlayerPlaceBlockMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerPlaceBlockRequest, com.tron.bridge.Player.PlayerPlaceBlockResponse> getPlayerPlaceBlockMethod;
    if ((getPlayerPlaceBlockMethod = BridgeGrpc.getPlayerPlaceBlockMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerPlaceBlockMethod = BridgeGrpc.getPlayerPlaceBlockMethod) == null) {
          BridgeGrpc.getPlayerPlaceBlockMethod = getPlayerPlaceBlockMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Player.PlayerPlaceBlockRequest, com.tron.bridge.Player.PlayerPlaceBlockResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerPlaceBlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerPlaceBlockRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerPlaceBlockResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerPlaceBlock"))
              .build();
        }
      }
    }
    return getPlayerPlaceBlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerBreakBlockRequest,
      com.tron.bridge.Player.PlayerBreakBlockResponse> getPlayerBreakBlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PlayerBreakBlock",
      requestType = com.tron.bridge.Player.PlayerBreakBlockRequest.class,
      responseType = com.tron.bridge.Player.PlayerBreakBlockResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerBreakBlockRequest,
      com.tron.bridge.Player.PlayerBreakBlockResponse> getPlayerBreakBlockMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Player.PlayerBreakBlockRequest, com.tron.bridge.Player.PlayerBreakBlockResponse> getPlayerBreakBlockMethod;
    if ((getPlayerBreakBlockMethod = BridgeGrpc.getPlayerBreakBlockMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getPlayerBreakBlockMethod = BridgeGrpc.getPlayerBreakBlockMethod) == null) {
          BridgeGrpc.getPlayerBreakBlockMethod = getPlayerBreakBlockMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Player.PlayerBreakBlockRequest, com.tron.bridge.Player.PlayerBreakBlockResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PlayerBreakBlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerBreakBlockRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Player.PlayerBreakBlockResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("PlayerBreakBlock"))
              .build();
        }
      }
    }
    return getPlayerBreakBlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyStartupRequest,
      com.tron.bridge.Server.ProxyStartupResponse> getProxyStartupMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ProxyStartup",
      requestType = com.tron.bridge.Server.ProxyStartupRequest.class,
      responseType = com.tron.bridge.Server.ProxyStartupResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyStartupRequest,
      com.tron.bridge.Server.ProxyStartupResponse> getProxyStartupMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyStartupRequest, com.tron.bridge.Server.ProxyStartupResponse> getProxyStartupMethod;
    if ((getProxyStartupMethod = BridgeGrpc.getProxyStartupMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getProxyStartupMethod = BridgeGrpc.getProxyStartupMethod) == null) {
          BridgeGrpc.getProxyStartupMethod = getProxyStartupMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.ProxyStartupRequest, com.tron.bridge.Server.ProxyStartupResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ProxyStartup"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ProxyStartupRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ProxyStartupResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ProxyStartup"))
              .build();
        }
      }
    }
    return getProxyStartupMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyShutdownRequest,
      com.tron.bridge.Server.ProxyShutdownResponse> getProxyShutdownMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ProxyShutdown",
      requestType = com.tron.bridge.Server.ProxyShutdownRequest.class,
      responseType = com.tron.bridge.Server.ProxyShutdownResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyShutdownRequest,
      com.tron.bridge.Server.ProxyShutdownResponse> getProxyShutdownMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.ProxyShutdownRequest, com.tron.bridge.Server.ProxyShutdownResponse> getProxyShutdownMethod;
    if ((getProxyShutdownMethod = BridgeGrpc.getProxyShutdownMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getProxyShutdownMethod = BridgeGrpc.getProxyShutdownMethod) == null) {
          BridgeGrpc.getProxyShutdownMethod = getProxyShutdownMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.ProxyShutdownRequest, com.tron.bridge.Server.ProxyShutdownResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ProxyShutdown"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ProxyShutdownRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ProxyShutdownResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ProxyShutdown"))
              .build();
        }
      }
    }
    return getProxyShutdownMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalStartupRequest,
      com.tron.bridge.Server.SurvivalStartupResponse> getSurvivalStartupMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SurvivalStartup",
      requestType = com.tron.bridge.Server.SurvivalStartupRequest.class,
      responseType = com.tron.bridge.Server.SurvivalStartupResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalStartupRequest,
      com.tron.bridge.Server.SurvivalStartupResponse> getSurvivalStartupMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalStartupRequest, com.tron.bridge.Server.SurvivalStartupResponse> getSurvivalStartupMethod;
    if ((getSurvivalStartupMethod = BridgeGrpc.getSurvivalStartupMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSurvivalStartupMethod = BridgeGrpc.getSurvivalStartupMethod) == null) {
          BridgeGrpc.getSurvivalStartupMethod = getSurvivalStartupMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.SurvivalStartupRequest, com.tron.bridge.Server.SurvivalStartupResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SurvivalStartup"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.SurvivalStartupRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.SurvivalStartupResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SurvivalStartup"))
              .build();
        }
      }
    }
    return getSurvivalStartupMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalShutdownRequest,
      com.tron.bridge.Server.SurvivalShutdownResponse> getSurvivalShutdownMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SurvivalShutdown",
      requestType = com.tron.bridge.Server.SurvivalShutdownRequest.class,
      responseType = com.tron.bridge.Server.SurvivalShutdownResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalShutdownRequest,
      com.tron.bridge.Server.SurvivalShutdownResponse> getSurvivalShutdownMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.SurvivalShutdownRequest, com.tron.bridge.Server.SurvivalShutdownResponse> getSurvivalShutdownMethod;
    if ((getSurvivalShutdownMethod = BridgeGrpc.getSurvivalShutdownMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getSurvivalShutdownMethod = BridgeGrpc.getSurvivalShutdownMethod) == null) {
          BridgeGrpc.getSurvivalShutdownMethod = getSurvivalShutdownMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.SurvivalShutdownRequest, com.tron.bridge.Server.SurvivalShutdownResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SurvivalShutdown"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.SurvivalShutdownRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.SurvivalShutdownResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("SurvivalShutdown"))
              .build();
        }
      }
    }
    return getSurvivalShutdownMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyStartupRequest,
      com.tron.bridge.Server.LobbyStartupResponse> getLobbyStartupMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LobbyStartup",
      requestType = com.tron.bridge.Server.LobbyStartupRequest.class,
      responseType = com.tron.bridge.Server.LobbyStartupResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyStartupRequest,
      com.tron.bridge.Server.LobbyStartupResponse> getLobbyStartupMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyStartupRequest, com.tron.bridge.Server.LobbyStartupResponse> getLobbyStartupMethod;
    if ((getLobbyStartupMethod = BridgeGrpc.getLobbyStartupMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getLobbyStartupMethod = BridgeGrpc.getLobbyStartupMethod) == null) {
          BridgeGrpc.getLobbyStartupMethod = getLobbyStartupMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.LobbyStartupRequest, com.tron.bridge.Server.LobbyStartupResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LobbyStartup"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.LobbyStartupRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.LobbyStartupResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("LobbyStartup"))
              .build();
        }
      }
    }
    return getLobbyStartupMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyShutdownRequest,
      com.tron.bridge.Server.LobbyShutdownResponse> getLobbyShutdownMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LobbyShutdown",
      requestType = com.tron.bridge.Server.LobbyShutdownRequest.class,
      responseType = com.tron.bridge.Server.LobbyShutdownResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyShutdownRequest,
      com.tron.bridge.Server.LobbyShutdownResponse> getLobbyShutdownMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.LobbyShutdownRequest, com.tron.bridge.Server.LobbyShutdownResponse> getLobbyShutdownMethod;
    if ((getLobbyShutdownMethod = BridgeGrpc.getLobbyShutdownMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getLobbyShutdownMethod = BridgeGrpc.getLobbyShutdownMethod) == null) {
          BridgeGrpc.getLobbyShutdownMethod = getLobbyShutdownMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.LobbyShutdownRequest, com.tron.bridge.Server.LobbyShutdownResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LobbyShutdown"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.LobbyShutdownRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.LobbyShutdownResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("LobbyShutdown"))
              .build();
        }
      }
    }
    return getLobbyShutdownMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest,
      com.tron.bridge.Server.ServerSendMessageResponse> getServerSendMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ServerSendMessage",
      requestType = com.tron.bridge.Server.ServerSubscribeRequest.class,
      responseType = com.tron.bridge.Server.ServerSendMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest,
      com.tron.bridge.Server.ServerSendMessageResponse> getServerSendMessageMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendMessageResponse> getServerSendMessageMethod;
    if ((getServerSendMessageMethod = BridgeGrpc.getServerSendMessageMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getServerSendMessageMethod = BridgeGrpc.getServerSendMessageMethod) == null) {
          BridgeGrpc.getServerSendMessageMethod = getServerSendMessageMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ServerSendMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ServerSubscribeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ServerSendMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ServerSendMessage"))
              .build();
        }
      }
    }
    return getServerSendMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest,
      com.tron.bridge.Server.ServerSendTitleResponse> getServerSendTitleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ServerSendTitle",
      requestType = com.tron.bridge.Server.ServerSubscribeRequest.class,
      responseType = com.tron.bridge.Server.ServerSendTitleResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest,
      com.tron.bridge.Server.ServerSendTitleResponse> getServerSendTitleMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendTitleResponse> getServerSendTitleMethod;
    if ((getServerSendTitleMethod = BridgeGrpc.getServerSendTitleMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getServerSendTitleMethod = BridgeGrpc.getServerSendTitleMethod) == null) {
          BridgeGrpc.getServerSendTitleMethod = getServerSendTitleMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.ServerSubscribeRequest, com.tron.bridge.Server.ServerSendTitleResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ServerSendTitle"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ServerSubscribeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.ServerSendTitleResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ServerSendTitle"))
              .build();
        }
      }
    }
    return getServerSendTitleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Server.MessageRequest,
      com.tron.bridge.Server.MessageResponse> getMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Message",
      requestType = com.tron.bridge.Server.MessageRequest.class,
      responseType = com.tron.bridge.Server.MessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Server.MessageRequest,
      com.tron.bridge.Server.MessageResponse> getMessageMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Server.MessageRequest, com.tron.bridge.Server.MessageResponse> getMessageMethod;
    if ((getMessageMethod = BridgeGrpc.getMessageMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getMessageMethod = BridgeGrpc.getMessageMethod) == null) {
          BridgeGrpc.getMessageMethod = getMessageMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Server.MessageRequest, com.tron.bridge.Server.MessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.BIDI_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Message"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.MessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Server.MessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("Message"))
              .build();
        }
      }
    }
    return getMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.tron.bridge.Security.ReportPlayerRequest,
      com.tron.bridge.Security.ReportPlayerResponse> getReportPlayerMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ReportPlayer",
      requestType = com.tron.bridge.Security.ReportPlayerRequest.class,
      responseType = com.tron.bridge.Security.ReportPlayerResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.tron.bridge.Security.ReportPlayerRequest,
      com.tron.bridge.Security.ReportPlayerResponse> getReportPlayerMethod() {
    io.grpc.MethodDescriptor<com.tron.bridge.Security.ReportPlayerRequest, com.tron.bridge.Security.ReportPlayerResponse> getReportPlayerMethod;
    if ((getReportPlayerMethod = BridgeGrpc.getReportPlayerMethod) == null) {
      synchronized (BridgeGrpc.class) {
        if ((getReportPlayerMethod = BridgeGrpc.getReportPlayerMethod) == null) {
          BridgeGrpc.getReportPlayerMethod = getReportPlayerMethod =
              io.grpc.MethodDescriptor.<com.tron.bridge.Security.ReportPlayerRequest, com.tron.bridge.Security.ReportPlayerResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ReportPlayer"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Security.ReportPlayerRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.tron.bridge.Security.ReportPlayerResponse.getDefaultInstance()))
              .setSchemaDescriptor(new BridgeMethodDescriptorSupplier("ReportPlayer"))
              .build();
        }
      }
    }
    return getReportPlayerMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static BridgeStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BridgeStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BridgeStub>() {
        @java.lang.Override
        public BridgeStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BridgeStub(channel, callOptions);
        }
      };
    return BridgeStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static BridgeBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BridgeBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BridgeBlockingStub>() {
        @java.lang.Override
        public BridgeBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BridgeBlockingStub(channel, callOptions);
        }
      };
    return BridgeBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static BridgeFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<BridgeFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<BridgeFutureStub>() {
        @java.lang.Override
        public BridgeFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new BridgeFutureStub(channel, callOptions);
        }
      };
    return BridgeFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     * <pre>
     * Session
     * </pre>
     */
    default void playerJoin(com.tron.bridge.Session.PlayerJoinRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerJoinResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerJoinMethod(), responseObserver);
    }

    /**
     */
    default void playerLeave(com.tron.bridge.Session.PlayerLeaveRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerLeaveResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerLeaveMethod(), responseObserver);
    }

    /**
     * <pre>
     * Balance
     * </pre>
     */
    default void getBalance(com.tron.bridge.Balance.GetBalanceRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Balance.GetBalanceResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetBalanceMethod(), responseObserver);
    }

    /**
     */
    default void transferBalance(com.tron.bridge.Balance.TransferBalanceRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Balance.TransferBalanceResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTransferBalanceMethod(), responseObserver);
    }

    /**
     * <pre>
     * Leaderboard
     * </pre>
     */
    default void overallLeaderboard(com.tron.bridge.Leaderboard.OverallLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.OverallLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getOverallLeaderboardMethod(), responseObserver);
    }

    /**
     */
    default void coinsLeaderboard(com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCoinsLeaderboardMethod(), responseObserver);
    }

    /**
     */
    default void teamsLeaderboard(com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTeamsLeaderboardMethod(), responseObserver);
    }

    /**
     */
    default void kdaLeaderboard(com.tron.bridge.Leaderboard.KdaLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KdaLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getKdaLeaderboardMethod(), responseObserver);
    }

    /**
     */
    default void deathsLeaderboard(com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeathsLeaderboardMethod(), responseObserver);
    }

    /**
     */
    default void killsLeaderboard(com.tron.bridge.Leaderboard.KillsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KillsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getKillsLeaderboardMethod(), responseObserver);
    }

    /**
     * <pre>
     * Friends
     * </pre>
     */
    default void getFriends(com.tron.bridge.Friends.GetFriendsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetFriendsMethod(), responseObserver);
    }

    /**
     */
    default void listFriends(com.tron.bridge.Friends.ListFriendsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListFriendsMethod(), responseObserver);
    }

    /**
     */
    default void sendFriendRequest(com.tron.bridge.Friends.SendFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.SendFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSendFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void acceptFriendRequest(com.tron.bridge.Friends.AcceptFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.AcceptFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAcceptFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void rejectFriendRequest(com.tron.bridge.Friends.RejectFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RejectFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRejectFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void getFriendRequests(com.tron.bridge.Friends.GetFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetFriendRequestsMethod(), responseObserver);
    }

    /**
     */
    default void listFriendRequests(com.tron.bridge.Friends.ListFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListFriendRequestsMethod(), responseObserver);
    }

    /**
     */
    default void removeFriend(com.tron.bridge.Friends.RemoveFriendRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveFriendResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveFriendMethod(), responseObserver);
    }

    /**
     * <pre>
     * Teams
     * </pre>
     */
    default void createTeam(com.tron.bridge.Teams.CreateTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.CreateTeamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateTeamMethod(), responseObserver);
    }

    /**
     */
    default void deleteTeam(com.tron.bridge.Teams.DeleteTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.DeleteTeamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteTeamMethod(), responseObserver);
    }

    /**
     */
    default void leaveTeam(com.tron.bridge.Teams.LeaveTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.LeaveTeamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLeaveTeamMethod(), responseObserver);
    }

    /**
     */
    default void joinTeam(com.tron.bridge.Teams.JoinTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.JoinTeamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getJoinTeamMethod(), responseObserver);
    }

    /**
     */
    default void sendTeamInvite(com.tron.bridge.Teams.SendTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.SendTeamInviteResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSendTeamInviteMethod(), responseObserver);
    }

    /**
     */
    default void acceptTeamInvite(com.tron.bridge.Teams.AcceptTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.AcceptTeamInviteResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAcceptTeamInviteMethod(), responseObserver);
    }

    /**
     */
    default void rejectTeamInvite(com.tron.bridge.Teams.RejectTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.RejectTeamInviteResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRejectTeamInviteMethod(), responseObserver);
    }

    /**
     */
    default void getTeamMembers(com.tron.bridge.Teams.GetTeamMembersRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetTeamMembersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetTeamMembersMethod(), responseObserver);
    }

    /**
     */
    default void removeTeamMember(com.tron.bridge.Friends.RemoveTeamMemberRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveTeamMemberResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveTeamMemberMethod(), responseObserver);
    }

    /**
     */
    default void promoteTeamMember(com.tron.bridge.Teams.PromoteTeamMemberRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.PromoteTeamMemberResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPromoteTeamMemberMethod(), responseObserver);
    }

    /**
     */
    default void getOpenTeams(com.tron.bridge.Teams.GetOpenTeamsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetOpenTeamsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetOpenTeamsMethod(), responseObserver);
    }

    /**
     * <pre>
     * Shop
     * </pre>
     */
    default void buyItem(com.tron.bridge.Shop.BuyItemRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.BuyItemResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBuyItemMethod(), responseObserver);
    }

    /**
     */
    default void sellItem(com.tron.bridge.Shop.SellItemRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.SellItemResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSellItemMethod(), responseObserver);
    }

    /**
     */
    default void getItems(com.tron.bridge.Shop.GetItemsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.GetItemsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetItemsMethod(), responseObserver);
    }

    /**
     * <pre>
     * Prefix
     * </pre>
     */
    default void buyPrefix(com.tron.bridge.Prefix.BuyPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.BuyPrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBuyPrefixMethod(), responseObserver);
    }

    /**
     */
    default void selectPrefix(com.tron.bridge.Prefix.SelectPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.SelectPrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSelectPrefixMethod(), responseObserver);
    }

    /**
     */
    default void getAllPrefix(com.tron.bridge.Prefix.GetAllPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetAllPrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetAllPrefixMethod(), responseObserver);
    }

    /**
     */
    default void getOwnedPrefix(com.tron.bridge.Prefix.GetOwnedPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetOwnedPrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetOwnedPrefixMethod(), responseObserver);
    }

    /**
     */
    default void getCurrentPrefix(com.tron.bridge.Prefix.GetCurrentPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetCurrentPrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetCurrentPrefixMethod(), responseObserver);
    }

    /**
     */
    default void createPrefix(com.tron.bridge.Prefix.CreatePrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.CreatePrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreatePrefixMethod(), responseObserver);
    }

    /**
     */
    default void deletePrefix(com.tron.bridge.Prefix.DeletePrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.DeletePrefixResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeletePrefixMethod(), responseObserver);
    }

    /**
     * <pre>
     * Player
     * </pre>
     */
    default void playerDeath(com.tron.bridge.Player.PlayerDeathRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerDeathResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerDeathMethod(), responseObserver);
    }

    /**
     */
    default void playerKill(com.tron.bridge.Player.PlayerKillRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerKillResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerKillMethod(), responseObserver);
    }

    /**
     */
    default void playerPlaceBlock(com.tron.bridge.Player.PlayerPlaceBlockRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerPlaceBlockResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerPlaceBlockMethod(), responseObserver);
    }

    /**
     */
    default void playerBreakBlock(com.tron.bridge.Player.PlayerBreakBlockRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerBreakBlockResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPlayerBreakBlockMethod(), responseObserver);
    }

    /**
     * <pre>
     * Server
     * </pre>
     */
    default void proxyStartup(com.tron.bridge.Server.ProxyStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyStartupResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getProxyStartupMethod(), responseObserver);
    }

    /**
     */
    default void proxyShutdown(com.tron.bridge.Server.ProxyShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyShutdownResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getProxyShutdownMethod(), responseObserver);
    }

    /**
     */
    default void survivalStartup(com.tron.bridge.Server.SurvivalStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalStartupResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSurvivalStartupMethod(), responseObserver);
    }

    /**
     */
    default void survivalShutdown(com.tron.bridge.Server.SurvivalShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalShutdownResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSurvivalShutdownMethod(), responseObserver);
    }

    /**
     */
    default void lobbyStartup(com.tron.bridge.Server.LobbyStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyStartupResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLobbyStartupMethod(), responseObserver);
    }

    /**
     */
    default void lobbyShutdown(com.tron.bridge.Server.LobbyShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyShutdownResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLobbyShutdownMethod(), responseObserver);
    }

    /**
     */
    default void serverSendMessage(com.tron.bridge.Server.ServerSubscribeRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getServerSendMessageMethod(), responseObserver);
    }

    /**
     */
    default void serverSendTitle(com.tron.bridge.Server.ServerSubscribeRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendTitleResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getServerSendTitleMethod(), responseObserver);
    }

    /**
     */
    default io.grpc.stub.StreamObserver<com.tron.bridge.Server.MessageRequest> message(
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.MessageResponse> responseObserver) {
      return io.grpc.stub.ServerCalls.asyncUnimplementedStreamingCall(getMessageMethod(), responseObserver);
    }

    /**
     * <pre>
     * Security
     * </pre>
     */
    default void reportPlayer(com.tron.bridge.Security.ReportPlayerRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Security.ReportPlayerResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getReportPlayerMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service Bridge.
   */
  public static abstract class BridgeImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return BridgeGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service Bridge.
   */
  public static final class BridgeStub
      extends io.grpc.stub.AbstractAsyncStub<BridgeStub> {
    private BridgeStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BridgeStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BridgeStub(channel, callOptions);
    }

    /**
     * <pre>
     * Session
     * </pre>
     */
    public void playerJoin(com.tron.bridge.Session.PlayerJoinRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerJoinResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerJoinMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void playerLeave(com.tron.bridge.Session.PlayerLeaveRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerLeaveResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerLeaveMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Balance
     * </pre>
     */
    public void getBalance(com.tron.bridge.Balance.GetBalanceRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Balance.GetBalanceResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetBalanceMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void transferBalance(com.tron.bridge.Balance.TransferBalanceRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Balance.TransferBalanceResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTransferBalanceMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Leaderboard
     * </pre>
     */
    public void overallLeaderboard(com.tron.bridge.Leaderboard.OverallLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.OverallLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getOverallLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void coinsLeaderboard(com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCoinsLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void teamsLeaderboard(com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTeamsLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void kdaLeaderboard(com.tron.bridge.Leaderboard.KdaLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KdaLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getKdaLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deathsLeaderboard(com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeathsLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void killsLeaderboard(com.tron.bridge.Leaderboard.KillsLeaderboardRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KillsLeaderboardResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getKillsLeaderboardMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Friends
     * </pre>
     */
    public void getFriends(com.tron.bridge.Friends.GetFriendsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetFriendsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listFriends(com.tron.bridge.Friends.ListFriendsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListFriendsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void sendFriendRequest(com.tron.bridge.Friends.SendFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.SendFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSendFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void acceptFriendRequest(com.tron.bridge.Friends.AcceptFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.AcceptFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAcceptFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void rejectFriendRequest(com.tron.bridge.Friends.RejectFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RejectFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRejectFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getFriendRequests(com.tron.bridge.Friends.GetFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetFriendRequestsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listFriendRequests(com.tron.bridge.Friends.ListFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListFriendRequestsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeFriend(com.tron.bridge.Friends.RemoveFriendRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveFriendResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveFriendMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Teams
     * </pre>
     */
    public void createTeam(com.tron.bridge.Teams.CreateTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.CreateTeamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateTeamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteTeam(com.tron.bridge.Teams.DeleteTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.DeleteTeamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteTeamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void leaveTeam(com.tron.bridge.Teams.LeaveTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.LeaveTeamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLeaveTeamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void joinTeam(com.tron.bridge.Teams.JoinTeamRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.JoinTeamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getJoinTeamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void sendTeamInvite(com.tron.bridge.Teams.SendTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.SendTeamInviteResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSendTeamInviteMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void acceptTeamInvite(com.tron.bridge.Teams.AcceptTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.AcceptTeamInviteResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAcceptTeamInviteMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void rejectTeamInvite(com.tron.bridge.Teams.RejectTeamInviteRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.RejectTeamInviteResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRejectTeamInviteMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getTeamMembers(com.tron.bridge.Teams.GetTeamMembersRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetTeamMembersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetTeamMembersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeTeamMember(com.tron.bridge.Friends.RemoveTeamMemberRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveTeamMemberResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveTeamMemberMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void promoteTeamMember(com.tron.bridge.Teams.PromoteTeamMemberRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.PromoteTeamMemberResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPromoteTeamMemberMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getOpenTeams(com.tron.bridge.Teams.GetOpenTeamsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetOpenTeamsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetOpenTeamsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Shop
     * </pre>
     */
    public void buyItem(com.tron.bridge.Shop.BuyItemRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.BuyItemResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBuyItemMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void sellItem(com.tron.bridge.Shop.SellItemRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.SellItemResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSellItemMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getItems(com.tron.bridge.Shop.GetItemsRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Shop.GetItemsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetItemsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Prefix
     * </pre>
     */
    public void buyPrefix(com.tron.bridge.Prefix.BuyPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.BuyPrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBuyPrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void selectPrefix(com.tron.bridge.Prefix.SelectPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.SelectPrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSelectPrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getAllPrefix(com.tron.bridge.Prefix.GetAllPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetAllPrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetAllPrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getOwnedPrefix(com.tron.bridge.Prefix.GetOwnedPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetOwnedPrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetOwnedPrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void getCurrentPrefix(com.tron.bridge.Prefix.GetCurrentPrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetCurrentPrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetCurrentPrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createPrefix(com.tron.bridge.Prefix.CreatePrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.CreatePrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreatePrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deletePrefix(com.tron.bridge.Prefix.DeletePrefixRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.DeletePrefixResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeletePrefixMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Player
     * </pre>
     */
    public void playerDeath(com.tron.bridge.Player.PlayerDeathRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerDeathResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerDeathMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void playerKill(com.tron.bridge.Player.PlayerKillRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerKillResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerKillMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void playerPlaceBlock(com.tron.bridge.Player.PlayerPlaceBlockRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerPlaceBlockResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerPlaceBlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void playerBreakBlock(com.tron.bridge.Player.PlayerBreakBlockRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerBreakBlockResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPlayerBreakBlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     * <pre>
     * Server
     * </pre>
     */
    public void proxyStartup(com.tron.bridge.Server.ProxyStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyStartupResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getProxyStartupMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void proxyShutdown(com.tron.bridge.Server.ProxyShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyShutdownResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getProxyShutdownMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void survivalStartup(com.tron.bridge.Server.SurvivalStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalStartupResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSurvivalStartupMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void survivalShutdown(com.tron.bridge.Server.SurvivalShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalShutdownResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSurvivalShutdownMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void lobbyStartup(com.tron.bridge.Server.LobbyStartupRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyStartupResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLobbyStartupMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void lobbyShutdown(com.tron.bridge.Server.LobbyShutdownRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyShutdownResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLobbyShutdownMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void serverSendMessage(com.tron.bridge.Server.ServerSubscribeRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getServerSendMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void serverSendTitle(com.tron.bridge.Server.ServerSubscribeRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendTitleResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getServerSendTitleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public io.grpc.stub.StreamObserver<com.tron.bridge.Server.MessageRequest> message(
        io.grpc.stub.StreamObserver<com.tron.bridge.Server.MessageResponse> responseObserver) {
      return io.grpc.stub.ClientCalls.asyncBidiStreamingCall(
          getChannel().newCall(getMessageMethod(), getCallOptions()), responseObserver);
    }

    /**
     * <pre>
     * Security
     * </pre>
     */
    public void reportPlayer(com.tron.bridge.Security.ReportPlayerRequest request,
        io.grpc.stub.StreamObserver<com.tron.bridge.Security.ReportPlayerResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getReportPlayerMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service Bridge.
   */
  public static final class BridgeBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<BridgeBlockingStub> {
    private BridgeBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BridgeBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BridgeBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     * Session
     * </pre>
     */
    public com.tron.bridge.Session.PlayerJoinResponse playerJoin(com.tron.bridge.Session.PlayerJoinRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerJoinMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Session.PlayerLeaveResponse playerLeave(com.tron.bridge.Session.PlayerLeaveRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerLeaveMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Balance
     * </pre>
     */
    public com.tron.bridge.Balance.GetBalanceResponse getBalance(com.tron.bridge.Balance.GetBalanceRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetBalanceMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Balance.TransferBalanceResponse transferBalance(com.tron.bridge.Balance.TransferBalanceRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTransferBalanceMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Leaderboard
     * </pre>
     */
    public com.tron.bridge.Leaderboard.OverallLeaderboardResponse overallLeaderboard(com.tron.bridge.Leaderboard.OverallLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getOverallLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Leaderboard.CoinsLeaderboardResponse coinsLeaderboard(com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCoinsLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Leaderboard.TeamsLeaderboardResponse teamsLeaderboard(com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTeamsLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Leaderboard.KdaLeaderboardResponse kdaLeaderboard(com.tron.bridge.Leaderboard.KdaLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getKdaLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Leaderboard.DeathsLeaderboardResponse deathsLeaderboard(com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeathsLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Leaderboard.KillsLeaderboardResponse killsLeaderboard(com.tron.bridge.Leaderboard.KillsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getKillsLeaderboardMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Friends
     * </pre>
     */
    public com.tron.bridge.Friends.GetFriendsResponse getFriends(com.tron.bridge.Friends.GetFriendsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetFriendsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.ListFriendsResponse listFriends(com.tron.bridge.Friends.ListFriendsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListFriendsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.SendFriendRequestResponse sendFriendRequest(com.tron.bridge.Friends.SendFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSendFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.AcceptFriendRequestResponse acceptFriendRequest(com.tron.bridge.Friends.AcceptFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAcceptFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.RejectFriendRequestResponse rejectFriendRequest(com.tron.bridge.Friends.RejectFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRejectFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.GetFriendRequestsResponse getFriendRequests(com.tron.bridge.Friends.GetFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetFriendRequestsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.ListFriendRequestsResponse listFriendRequests(com.tron.bridge.Friends.ListFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListFriendRequestsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.RemoveFriendResponse removeFriend(com.tron.bridge.Friends.RemoveFriendRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveFriendMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Teams
     * </pre>
     */
    public com.tron.bridge.Teams.CreateTeamResponse createTeam(com.tron.bridge.Teams.CreateTeamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateTeamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.DeleteTeamResponse deleteTeam(com.tron.bridge.Teams.DeleteTeamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteTeamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.LeaveTeamResponse leaveTeam(com.tron.bridge.Teams.LeaveTeamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLeaveTeamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.JoinTeamResponse joinTeam(com.tron.bridge.Teams.JoinTeamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getJoinTeamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.SendTeamInviteResponse sendTeamInvite(com.tron.bridge.Teams.SendTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSendTeamInviteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.AcceptTeamInviteResponse acceptTeamInvite(com.tron.bridge.Teams.AcceptTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAcceptTeamInviteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.RejectTeamInviteResponse rejectTeamInvite(com.tron.bridge.Teams.RejectTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRejectTeamInviteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.GetTeamMembersResponse getTeamMembers(com.tron.bridge.Teams.GetTeamMembersRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetTeamMembersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Friends.RemoveTeamMemberResponse removeTeamMember(com.tron.bridge.Friends.RemoveTeamMemberRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveTeamMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.PromoteTeamMemberResponse promoteTeamMember(com.tron.bridge.Teams.PromoteTeamMemberRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPromoteTeamMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Teams.GetOpenTeamsResponse getOpenTeams(com.tron.bridge.Teams.GetOpenTeamsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetOpenTeamsMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Shop
     * </pre>
     */
    public com.tron.bridge.Shop.BuyItemResponse buyItem(com.tron.bridge.Shop.BuyItemRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBuyItemMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Shop.SellItemResponse sellItem(com.tron.bridge.Shop.SellItemRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSellItemMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Shop.GetItemsResponse getItems(com.tron.bridge.Shop.GetItemsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetItemsMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Prefix
     * </pre>
     */
    public com.tron.bridge.Prefix.BuyPrefixResponse buyPrefix(com.tron.bridge.Prefix.BuyPrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBuyPrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.SelectPrefixResponse selectPrefix(com.tron.bridge.Prefix.SelectPrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSelectPrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.GetAllPrefixResponse getAllPrefix(com.tron.bridge.Prefix.GetAllPrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetAllPrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.GetOwnedPrefixResponse getOwnedPrefix(com.tron.bridge.Prefix.GetOwnedPrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetOwnedPrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.GetCurrentPrefixResponse getCurrentPrefix(com.tron.bridge.Prefix.GetCurrentPrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetCurrentPrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.CreatePrefixResponse createPrefix(com.tron.bridge.Prefix.CreatePrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreatePrefixMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Prefix.DeletePrefixResponse deletePrefix(com.tron.bridge.Prefix.DeletePrefixRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeletePrefixMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Player
     * </pre>
     */
    public com.tron.bridge.Player.PlayerDeathResponse playerDeath(com.tron.bridge.Player.PlayerDeathRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerDeathMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Player.PlayerKillResponse playerKill(com.tron.bridge.Player.PlayerKillRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerKillMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Player.PlayerPlaceBlockResponse playerPlaceBlock(com.tron.bridge.Player.PlayerPlaceBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerPlaceBlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Player.PlayerBreakBlockResponse playerBreakBlock(com.tron.bridge.Player.PlayerBreakBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPlayerBreakBlockMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Server
     * </pre>
     */
    public com.tron.bridge.Server.ProxyStartupResponse proxyStartup(com.tron.bridge.Server.ProxyStartupRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getProxyStartupMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Server.ProxyShutdownResponse proxyShutdown(com.tron.bridge.Server.ProxyShutdownRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getProxyShutdownMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Server.SurvivalStartupResponse survivalStartup(com.tron.bridge.Server.SurvivalStartupRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSurvivalStartupMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Server.SurvivalShutdownResponse survivalShutdown(com.tron.bridge.Server.SurvivalShutdownRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSurvivalShutdownMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Server.LobbyStartupResponse lobbyStartup(com.tron.bridge.Server.LobbyStartupRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLobbyStartupMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.tron.bridge.Server.LobbyShutdownResponse lobbyShutdown(com.tron.bridge.Server.LobbyShutdownRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLobbyShutdownMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.tron.bridge.Server.ServerSendMessageResponse> serverSendMessage(
        com.tron.bridge.Server.ServerSubscribeRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getServerSendMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.tron.bridge.Server.ServerSendTitleResponse> serverSendTitle(
        com.tron.bridge.Server.ServerSubscribeRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getServerSendTitleMethod(), getCallOptions(), request);
    }

    /**
     * <pre>
     * Security
     * </pre>
     */
    public com.tron.bridge.Security.ReportPlayerResponse reportPlayer(com.tron.bridge.Security.ReportPlayerRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getReportPlayerMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service Bridge.
   */
  public static final class BridgeFutureStub
      extends io.grpc.stub.AbstractFutureStub<BridgeFutureStub> {
    private BridgeFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected BridgeFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new BridgeFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     * Session
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Session.PlayerJoinResponse> playerJoin(
        com.tron.bridge.Session.PlayerJoinRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerJoinMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Session.PlayerLeaveResponse> playerLeave(
        com.tron.bridge.Session.PlayerLeaveRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerLeaveMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Balance
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Balance.GetBalanceResponse> getBalance(
        com.tron.bridge.Balance.GetBalanceRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetBalanceMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Balance.TransferBalanceResponse> transferBalance(
        com.tron.bridge.Balance.TransferBalanceRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTransferBalanceMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Leaderboard
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.OverallLeaderboardResponse> overallLeaderboard(
        com.tron.bridge.Leaderboard.OverallLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getOverallLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.CoinsLeaderboardResponse> coinsLeaderboard(
        com.tron.bridge.Leaderboard.CoinsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCoinsLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.TeamsLeaderboardResponse> teamsLeaderboard(
        com.tron.bridge.Leaderboard.TeamsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTeamsLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.KdaLeaderboardResponse> kdaLeaderboard(
        com.tron.bridge.Leaderboard.KdaLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getKdaLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.DeathsLeaderboardResponse> deathsLeaderboard(
        com.tron.bridge.Leaderboard.DeathsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeathsLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Leaderboard.KillsLeaderboardResponse> killsLeaderboard(
        com.tron.bridge.Leaderboard.KillsLeaderboardRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getKillsLeaderboardMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Friends
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.GetFriendsResponse> getFriends(
        com.tron.bridge.Friends.GetFriendsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetFriendsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.ListFriendsResponse> listFriends(
        com.tron.bridge.Friends.ListFriendsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListFriendsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.SendFriendRequestResponse> sendFriendRequest(
        com.tron.bridge.Friends.SendFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSendFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.AcceptFriendRequestResponse> acceptFriendRequest(
        com.tron.bridge.Friends.AcceptFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAcceptFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.RejectFriendRequestResponse> rejectFriendRequest(
        com.tron.bridge.Friends.RejectFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRejectFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.GetFriendRequestsResponse> getFriendRequests(
        com.tron.bridge.Friends.GetFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetFriendRequestsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.ListFriendRequestsResponse> listFriendRequests(
        com.tron.bridge.Friends.ListFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListFriendRequestsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.RemoveFriendResponse> removeFriend(
        com.tron.bridge.Friends.RemoveFriendRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveFriendMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Teams
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.CreateTeamResponse> createTeam(
        com.tron.bridge.Teams.CreateTeamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateTeamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.DeleteTeamResponse> deleteTeam(
        com.tron.bridge.Teams.DeleteTeamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteTeamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.LeaveTeamResponse> leaveTeam(
        com.tron.bridge.Teams.LeaveTeamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLeaveTeamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.JoinTeamResponse> joinTeam(
        com.tron.bridge.Teams.JoinTeamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getJoinTeamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.SendTeamInviteResponse> sendTeamInvite(
        com.tron.bridge.Teams.SendTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSendTeamInviteMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.AcceptTeamInviteResponse> acceptTeamInvite(
        com.tron.bridge.Teams.AcceptTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAcceptTeamInviteMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.RejectTeamInviteResponse> rejectTeamInvite(
        com.tron.bridge.Teams.RejectTeamInviteRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRejectTeamInviteMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.GetTeamMembersResponse> getTeamMembers(
        com.tron.bridge.Teams.GetTeamMembersRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetTeamMembersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Friends.RemoveTeamMemberResponse> removeTeamMember(
        com.tron.bridge.Friends.RemoveTeamMemberRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveTeamMemberMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.PromoteTeamMemberResponse> promoteTeamMember(
        com.tron.bridge.Teams.PromoteTeamMemberRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPromoteTeamMemberMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Teams.GetOpenTeamsResponse> getOpenTeams(
        com.tron.bridge.Teams.GetOpenTeamsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetOpenTeamsMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Shop
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Shop.BuyItemResponse> buyItem(
        com.tron.bridge.Shop.BuyItemRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBuyItemMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Shop.SellItemResponse> sellItem(
        com.tron.bridge.Shop.SellItemRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSellItemMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Shop.GetItemsResponse> getItems(
        com.tron.bridge.Shop.GetItemsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetItemsMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Prefix
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.BuyPrefixResponse> buyPrefix(
        com.tron.bridge.Prefix.BuyPrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBuyPrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.SelectPrefixResponse> selectPrefix(
        com.tron.bridge.Prefix.SelectPrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSelectPrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.GetAllPrefixResponse> getAllPrefix(
        com.tron.bridge.Prefix.GetAllPrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetAllPrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.GetOwnedPrefixResponse> getOwnedPrefix(
        com.tron.bridge.Prefix.GetOwnedPrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetOwnedPrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.GetCurrentPrefixResponse> getCurrentPrefix(
        com.tron.bridge.Prefix.GetCurrentPrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetCurrentPrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.CreatePrefixResponse> createPrefix(
        com.tron.bridge.Prefix.CreatePrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreatePrefixMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Prefix.DeletePrefixResponse> deletePrefix(
        com.tron.bridge.Prefix.DeletePrefixRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeletePrefixMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Player
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Player.PlayerDeathResponse> playerDeath(
        com.tron.bridge.Player.PlayerDeathRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerDeathMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Player.PlayerKillResponse> playerKill(
        com.tron.bridge.Player.PlayerKillRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerKillMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Player.PlayerPlaceBlockResponse> playerPlaceBlock(
        com.tron.bridge.Player.PlayerPlaceBlockRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerPlaceBlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Player.PlayerBreakBlockResponse> playerBreakBlock(
        com.tron.bridge.Player.PlayerBreakBlockRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPlayerBreakBlockMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Server
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.ProxyStartupResponse> proxyStartup(
        com.tron.bridge.Server.ProxyStartupRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getProxyStartupMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.ProxyShutdownResponse> proxyShutdown(
        com.tron.bridge.Server.ProxyShutdownRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getProxyShutdownMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.SurvivalStartupResponse> survivalStartup(
        com.tron.bridge.Server.SurvivalStartupRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSurvivalStartupMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.SurvivalShutdownResponse> survivalShutdown(
        com.tron.bridge.Server.SurvivalShutdownRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSurvivalShutdownMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.LobbyStartupResponse> lobbyStartup(
        com.tron.bridge.Server.LobbyStartupRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLobbyStartupMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Server.LobbyShutdownResponse> lobbyShutdown(
        com.tron.bridge.Server.LobbyShutdownRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLobbyShutdownMethod(), getCallOptions()), request);
    }

    /**
     * <pre>
     * Security
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<com.tron.bridge.Security.ReportPlayerResponse> reportPlayer(
        com.tron.bridge.Security.ReportPlayerRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getReportPlayerMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_PLAYER_JOIN = 0;
  private static final int METHODID_PLAYER_LEAVE = 1;
  private static final int METHODID_GET_BALANCE = 2;
  private static final int METHODID_TRANSFER_BALANCE = 3;
  private static final int METHODID_OVERALL_LEADERBOARD = 4;
  private static final int METHODID_COINS_LEADERBOARD = 5;
  private static final int METHODID_TEAMS_LEADERBOARD = 6;
  private static final int METHODID_KDA_LEADERBOARD = 7;
  private static final int METHODID_DEATHS_LEADERBOARD = 8;
  private static final int METHODID_KILLS_LEADERBOARD = 9;
  private static final int METHODID_GET_FRIENDS = 10;
  private static final int METHODID_LIST_FRIENDS = 11;
  private static final int METHODID_SEND_FRIEND_REQUEST = 12;
  private static final int METHODID_ACCEPT_FRIEND_REQUEST = 13;
  private static final int METHODID_REJECT_FRIEND_REQUEST = 14;
  private static final int METHODID_GET_FRIEND_REQUESTS = 15;
  private static final int METHODID_LIST_FRIEND_REQUESTS = 16;
  private static final int METHODID_REMOVE_FRIEND = 17;
  private static final int METHODID_CREATE_TEAM = 18;
  private static final int METHODID_DELETE_TEAM = 19;
  private static final int METHODID_LEAVE_TEAM = 20;
  private static final int METHODID_JOIN_TEAM = 21;
  private static final int METHODID_SEND_TEAM_INVITE = 22;
  private static final int METHODID_ACCEPT_TEAM_INVITE = 23;
  private static final int METHODID_REJECT_TEAM_INVITE = 24;
  private static final int METHODID_GET_TEAM_MEMBERS = 25;
  private static final int METHODID_REMOVE_TEAM_MEMBER = 26;
  private static final int METHODID_PROMOTE_TEAM_MEMBER = 27;
  private static final int METHODID_GET_OPEN_TEAMS = 28;
  private static final int METHODID_BUY_ITEM = 29;
  private static final int METHODID_SELL_ITEM = 30;
  private static final int METHODID_GET_ITEMS = 31;
  private static final int METHODID_BUY_PREFIX = 32;
  private static final int METHODID_SELECT_PREFIX = 33;
  private static final int METHODID_GET_ALL_PREFIX = 34;
  private static final int METHODID_GET_OWNED_PREFIX = 35;
  private static final int METHODID_GET_CURRENT_PREFIX = 36;
  private static final int METHODID_CREATE_PREFIX = 37;
  private static final int METHODID_DELETE_PREFIX = 38;
  private static final int METHODID_PLAYER_DEATH = 39;
  private static final int METHODID_PLAYER_KILL = 40;
  private static final int METHODID_PLAYER_PLACE_BLOCK = 41;
  private static final int METHODID_PLAYER_BREAK_BLOCK = 42;
  private static final int METHODID_PROXY_STARTUP = 43;
  private static final int METHODID_PROXY_SHUTDOWN = 44;
  private static final int METHODID_SURVIVAL_STARTUP = 45;
  private static final int METHODID_SURVIVAL_SHUTDOWN = 46;
  private static final int METHODID_LOBBY_STARTUP = 47;
  private static final int METHODID_LOBBY_SHUTDOWN = 48;
  private static final int METHODID_SERVER_SEND_MESSAGE = 49;
  private static final int METHODID_SERVER_SEND_TITLE = 50;
  private static final int METHODID_REPORT_PLAYER = 51;
  private static final int METHODID_MESSAGE = 52;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_PLAYER_JOIN:
          serviceImpl.playerJoin((com.tron.bridge.Session.PlayerJoinRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerJoinResponse>) responseObserver);
          break;
        case METHODID_PLAYER_LEAVE:
          serviceImpl.playerLeave((com.tron.bridge.Session.PlayerLeaveRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Session.PlayerLeaveResponse>) responseObserver);
          break;
        case METHODID_GET_BALANCE:
          serviceImpl.getBalance((com.tron.bridge.Balance.GetBalanceRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Balance.GetBalanceResponse>) responseObserver);
          break;
        case METHODID_TRANSFER_BALANCE:
          serviceImpl.transferBalance((com.tron.bridge.Balance.TransferBalanceRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Balance.TransferBalanceResponse>) responseObserver);
          break;
        case METHODID_OVERALL_LEADERBOARD:
          serviceImpl.overallLeaderboard((com.tron.bridge.Leaderboard.OverallLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.OverallLeaderboardResponse>) responseObserver);
          break;
        case METHODID_COINS_LEADERBOARD:
          serviceImpl.coinsLeaderboard((com.tron.bridge.Leaderboard.CoinsLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.CoinsLeaderboardResponse>) responseObserver);
          break;
        case METHODID_TEAMS_LEADERBOARD:
          serviceImpl.teamsLeaderboard((com.tron.bridge.Leaderboard.TeamsLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.TeamsLeaderboardResponse>) responseObserver);
          break;
        case METHODID_KDA_LEADERBOARD:
          serviceImpl.kdaLeaderboard((com.tron.bridge.Leaderboard.KdaLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KdaLeaderboardResponse>) responseObserver);
          break;
        case METHODID_DEATHS_LEADERBOARD:
          serviceImpl.deathsLeaderboard((com.tron.bridge.Leaderboard.DeathsLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.DeathsLeaderboardResponse>) responseObserver);
          break;
        case METHODID_KILLS_LEADERBOARD:
          serviceImpl.killsLeaderboard((com.tron.bridge.Leaderboard.KillsLeaderboardRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Leaderboard.KillsLeaderboardResponse>) responseObserver);
          break;
        case METHODID_GET_FRIENDS:
          serviceImpl.getFriends((com.tron.bridge.Friends.GetFriendsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendsResponse>) responseObserver);
          break;
        case METHODID_LIST_FRIENDS:
          serviceImpl.listFriends((com.tron.bridge.Friends.ListFriendsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendsResponse>) responseObserver);
          break;
        case METHODID_SEND_FRIEND_REQUEST:
          serviceImpl.sendFriendRequest((com.tron.bridge.Friends.SendFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.SendFriendRequestResponse>) responseObserver);
          break;
        case METHODID_ACCEPT_FRIEND_REQUEST:
          serviceImpl.acceptFriendRequest((com.tron.bridge.Friends.AcceptFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.AcceptFriendRequestResponse>) responseObserver);
          break;
        case METHODID_REJECT_FRIEND_REQUEST:
          serviceImpl.rejectFriendRequest((com.tron.bridge.Friends.RejectFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RejectFriendRequestResponse>) responseObserver);
          break;
        case METHODID_GET_FRIEND_REQUESTS:
          serviceImpl.getFriendRequests((com.tron.bridge.Friends.GetFriendRequestsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.GetFriendRequestsResponse>) responseObserver);
          break;
        case METHODID_LIST_FRIEND_REQUESTS:
          serviceImpl.listFriendRequests((com.tron.bridge.Friends.ListFriendRequestsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.ListFriendRequestsResponse>) responseObserver);
          break;
        case METHODID_REMOVE_FRIEND:
          serviceImpl.removeFriend((com.tron.bridge.Friends.RemoveFriendRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveFriendResponse>) responseObserver);
          break;
        case METHODID_CREATE_TEAM:
          serviceImpl.createTeam((com.tron.bridge.Teams.CreateTeamRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.CreateTeamResponse>) responseObserver);
          break;
        case METHODID_DELETE_TEAM:
          serviceImpl.deleteTeam((com.tron.bridge.Teams.DeleteTeamRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.DeleteTeamResponse>) responseObserver);
          break;
        case METHODID_LEAVE_TEAM:
          serviceImpl.leaveTeam((com.tron.bridge.Teams.LeaveTeamRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.LeaveTeamResponse>) responseObserver);
          break;
        case METHODID_JOIN_TEAM:
          serviceImpl.joinTeam((com.tron.bridge.Teams.JoinTeamRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.JoinTeamResponse>) responseObserver);
          break;
        case METHODID_SEND_TEAM_INVITE:
          serviceImpl.sendTeamInvite((com.tron.bridge.Teams.SendTeamInviteRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.SendTeamInviteResponse>) responseObserver);
          break;
        case METHODID_ACCEPT_TEAM_INVITE:
          serviceImpl.acceptTeamInvite((com.tron.bridge.Teams.AcceptTeamInviteRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.AcceptTeamInviteResponse>) responseObserver);
          break;
        case METHODID_REJECT_TEAM_INVITE:
          serviceImpl.rejectTeamInvite((com.tron.bridge.Teams.RejectTeamInviteRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.RejectTeamInviteResponse>) responseObserver);
          break;
        case METHODID_GET_TEAM_MEMBERS:
          serviceImpl.getTeamMembers((com.tron.bridge.Teams.GetTeamMembersRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetTeamMembersResponse>) responseObserver);
          break;
        case METHODID_REMOVE_TEAM_MEMBER:
          serviceImpl.removeTeamMember((com.tron.bridge.Friends.RemoveTeamMemberRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Friends.RemoveTeamMemberResponse>) responseObserver);
          break;
        case METHODID_PROMOTE_TEAM_MEMBER:
          serviceImpl.promoteTeamMember((com.tron.bridge.Teams.PromoteTeamMemberRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.PromoteTeamMemberResponse>) responseObserver);
          break;
        case METHODID_GET_OPEN_TEAMS:
          serviceImpl.getOpenTeams((com.tron.bridge.Teams.GetOpenTeamsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Teams.GetOpenTeamsResponse>) responseObserver);
          break;
        case METHODID_BUY_ITEM:
          serviceImpl.buyItem((com.tron.bridge.Shop.BuyItemRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Shop.BuyItemResponse>) responseObserver);
          break;
        case METHODID_SELL_ITEM:
          serviceImpl.sellItem((com.tron.bridge.Shop.SellItemRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Shop.SellItemResponse>) responseObserver);
          break;
        case METHODID_GET_ITEMS:
          serviceImpl.getItems((com.tron.bridge.Shop.GetItemsRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Shop.GetItemsResponse>) responseObserver);
          break;
        case METHODID_BUY_PREFIX:
          serviceImpl.buyPrefix((com.tron.bridge.Prefix.BuyPrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.BuyPrefixResponse>) responseObserver);
          break;
        case METHODID_SELECT_PREFIX:
          serviceImpl.selectPrefix((com.tron.bridge.Prefix.SelectPrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.SelectPrefixResponse>) responseObserver);
          break;
        case METHODID_GET_ALL_PREFIX:
          serviceImpl.getAllPrefix((com.tron.bridge.Prefix.GetAllPrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetAllPrefixResponse>) responseObserver);
          break;
        case METHODID_GET_OWNED_PREFIX:
          serviceImpl.getOwnedPrefix((com.tron.bridge.Prefix.GetOwnedPrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetOwnedPrefixResponse>) responseObserver);
          break;
        case METHODID_GET_CURRENT_PREFIX:
          serviceImpl.getCurrentPrefix((com.tron.bridge.Prefix.GetCurrentPrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.GetCurrentPrefixResponse>) responseObserver);
          break;
        case METHODID_CREATE_PREFIX:
          serviceImpl.createPrefix((com.tron.bridge.Prefix.CreatePrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.CreatePrefixResponse>) responseObserver);
          break;
        case METHODID_DELETE_PREFIX:
          serviceImpl.deletePrefix((com.tron.bridge.Prefix.DeletePrefixRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Prefix.DeletePrefixResponse>) responseObserver);
          break;
        case METHODID_PLAYER_DEATH:
          serviceImpl.playerDeath((com.tron.bridge.Player.PlayerDeathRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerDeathResponse>) responseObserver);
          break;
        case METHODID_PLAYER_KILL:
          serviceImpl.playerKill((com.tron.bridge.Player.PlayerKillRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerKillResponse>) responseObserver);
          break;
        case METHODID_PLAYER_PLACE_BLOCK:
          serviceImpl.playerPlaceBlock((com.tron.bridge.Player.PlayerPlaceBlockRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerPlaceBlockResponse>) responseObserver);
          break;
        case METHODID_PLAYER_BREAK_BLOCK:
          serviceImpl.playerBreakBlock((com.tron.bridge.Player.PlayerBreakBlockRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Player.PlayerBreakBlockResponse>) responseObserver);
          break;
        case METHODID_PROXY_STARTUP:
          serviceImpl.proxyStartup((com.tron.bridge.Server.ProxyStartupRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyStartupResponse>) responseObserver);
          break;
        case METHODID_PROXY_SHUTDOWN:
          serviceImpl.proxyShutdown((com.tron.bridge.Server.ProxyShutdownRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.ProxyShutdownResponse>) responseObserver);
          break;
        case METHODID_SURVIVAL_STARTUP:
          serviceImpl.survivalStartup((com.tron.bridge.Server.SurvivalStartupRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalStartupResponse>) responseObserver);
          break;
        case METHODID_SURVIVAL_SHUTDOWN:
          serviceImpl.survivalShutdown((com.tron.bridge.Server.SurvivalShutdownRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.SurvivalShutdownResponse>) responseObserver);
          break;
        case METHODID_LOBBY_STARTUP:
          serviceImpl.lobbyStartup((com.tron.bridge.Server.LobbyStartupRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyStartupResponse>) responseObserver);
          break;
        case METHODID_LOBBY_SHUTDOWN:
          serviceImpl.lobbyShutdown((com.tron.bridge.Server.LobbyShutdownRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.LobbyShutdownResponse>) responseObserver);
          break;
        case METHODID_SERVER_SEND_MESSAGE:
          serviceImpl.serverSendMessage((com.tron.bridge.Server.ServerSubscribeRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendMessageResponse>) responseObserver);
          break;
        case METHODID_SERVER_SEND_TITLE:
          serviceImpl.serverSendTitle((com.tron.bridge.Server.ServerSubscribeRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.ServerSendTitleResponse>) responseObserver);
          break;
        case METHODID_REPORT_PLAYER:
          serviceImpl.reportPlayer((com.tron.bridge.Security.ReportPlayerRequest) request,
              (io.grpc.stub.StreamObserver<com.tron.bridge.Security.ReportPlayerResponse>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_MESSAGE:
          return (io.grpc.stub.StreamObserver<Req>) serviceImpl.message(
              (io.grpc.stub.StreamObserver<com.tron.bridge.Server.MessageResponse>) responseObserver);
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getPlayerJoinMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Session.PlayerJoinRequest,
              com.tron.bridge.Session.PlayerJoinResponse>(
                service, METHODID_PLAYER_JOIN)))
        .addMethod(
          getPlayerLeaveMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Session.PlayerLeaveRequest,
              com.tron.bridge.Session.PlayerLeaveResponse>(
                service, METHODID_PLAYER_LEAVE)))
        .addMethod(
          getGetBalanceMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Balance.GetBalanceRequest,
              com.tron.bridge.Balance.GetBalanceResponse>(
                service, METHODID_GET_BALANCE)))
        .addMethod(
          getTransferBalanceMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Balance.TransferBalanceRequest,
              com.tron.bridge.Balance.TransferBalanceResponse>(
                service, METHODID_TRANSFER_BALANCE)))
        .addMethod(
          getOverallLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.OverallLeaderboardRequest,
              com.tron.bridge.Leaderboard.OverallLeaderboardResponse>(
                service, METHODID_OVERALL_LEADERBOARD)))
        .addMethod(
          getCoinsLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.CoinsLeaderboardRequest,
              com.tron.bridge.Leaderboard.CoinsLeaderboardResponse>(
                service, METHODID_COINS_LEADERBOARD)))
        .addMethod(
          getTeamsLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.TeamsLeaderboardRequest,
              com.tron.bridge.Leaderboard.TeamsLeaderboardResponse>(
                service, METHODID_TEAMS_LEADERBOARD)))
        .addMethod(
          getKdaLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.KdaLeaderboardRequest,
              com.tron.bridge.Leaderboard.KdaLeaderboardResponse>(
                service, METHODID_KDA_LEADERBOARD)))
        .addMethod(
          getDeathsLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.DeathsLeaderboardRequest,
              com.tron.bridge.Leaderboard.DeathsLeaderboardResponse>(
                service, METHODID_DEATHS_LEADERBOARD)))
        .addMethod(
          getKillsLeaderboardMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Leaderboard.KillsLeaderboardRequest,
              com.tron.bridge.Leaderboard.KillsLeaderboardResponse>(
                service, METHODID_KILLS_LEADERBOARD)))
        .addMethod(
          getGetFriendsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.GetFriendsRequest,
              com.tron.bridge.Friends.GetFriendsResponse>(
                service, METHODID_GET_FRIENDS)))
        .addMethod(
          getListFriendsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.ListFriendsRequest,
              com.tron.bridge.Friends.ListFriendsResponse>(
                service, METHODID_LIST_FRIENDS)))
        .addMethod(
          getSendFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.SendFriendRequestRequest,
              com.tron.bridge.Friends.SendFriendRequestResponse>(
                service, METHODID_SEND_FRIEND_REQUEST)))
        .addMethod(
          getAcceptFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.AcceptFriendRequestRequest,
              com.tron.bridge.Friends.AcceptFriendRequestResponse>(
                service, METHODID_ACCEPT_FRIEND_REQUEST)))
        .addMethod(
          getRejectFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.RejectFriendRequestRequest,
              com.tron.bridge.Friends.RejectFriendRequestResponse>(
                service, METHODID_REJECT_FRIEND_REQUEST)))
        .addMethod(
          getGetFriendRequestsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.GetFriendRequestsRequest,
              com.tron.bridge.Friends.GetFriendRequestsResponse>(
                service, METHODID_GET_FRIEND_REQUESTS)))
        .addMethod(
          getListFriendRequestsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.ListFriendRequestsRequest,
              com.tron.bridge.Friends.ListFriendRequestsResponse>(
                service, METHODID_LIST_FRIEND_REQUESTS)))
        .addMethod(
          getRemoveFriendMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.RemoveFriendRequest,
              com.tron.bridge.Friends.RemoveFriendResponse>(
                service, METHODID_REMOVE_FRIEND)))
        .addMethod(
          getCreateTeamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.CreateTeamRequest,
              com.tron.bridge.Teams.CreateTeamResponse>(
                service, METHODID_CREATE_TEAM)))
        .addMethod(
          getDeleteTeamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.DeleteTeamRequest,
              com.tron.bridge.Teams.DeleteTeamResponse>(
                service, METHODID_DELETE_TEAM)))
        .addMethod(
          getLeaveTeamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.LeaveTeamRequest,
              com.tron.bridge.Teams.LeaveTeamResponse>(
                service, METHODID_LEAVE_TEAM)))
        .addMethod(
          getJoinTeamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.JoinTeamRequest,
              com.tron.bridge.Teams.JoinTeamResponse>(
                service, METHODID_JOIN_TEAM)))
        .addMethod(
          getSendTeamInviteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.SendTeamInviteRequest,
              com.tron.bridge.Teams.SendTeamInviteResponse>(
                service, METHODID_SEND_TEAM_INVITE)))
        .addMethod(
          getAcceptTeamInviteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.AcceptTeamInviteRequest,
              com.tron.bridge.Teams.AcceptTeamInviteResponse>(
                service, METHODID_ACCEPT_TEAM_INVITE)))
        .addMethod(
          getRejectTeamInviteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.RejectTeamInviteRequest,
              com.tron.bridge.Teams.RejectTeamInviteResponse>(
                service, METHODID_REJECT_TEAM_INVITE)))
        .addMethod(
          getGetTeamMembersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.GetTeamMembersRequest,
              com.tron.bridge.Teams.GetTeamMembersResponse>(
                service, METHODID_GET_TEAM_MEMBERS)))
        .addMethod(
          getRemoveTeamMemberMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Friends.RemoveTeamMemberRequest,
              com.tron.bridge.Friends.RemoveTeamMemberResponse>(
                service, METHODID_REMOVE_TEAM_MEMBER)))
        .addMethod(
          getPromoteTeamMemberMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.PromoteTeamMemberRequest,
              com.tron.bridge.Teams.PromoteTeamMemberResponse>(
                service, METHODID_PROMOTE_TEAM_MEMBER)))
        .addMethod(
          getGetOpenTeamsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Teams.GetOpenTeamsRequest,
              com.tron.bridge.Teams.GetOpenTeamsResponse>(
                service, METHODID_GET_OPEN_TEAMS)))
        .addMethod(
          getBuyItemMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Shop.BuyItemRequest,
              com.tron.bridge.Shop.BuyItemResponse>(
                service, METHODID_BUY_ITEM)))
        .addMethod(
          getSellItemMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Shop.SellItemRequest,
              com.tron.bridge.Shop.SellItemResponse>(
                service, METHODID_SELL_ITEM)))
        .addMethod(
          getGetItemsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Shop.GetItemsRequest,
              com.tron.bridge.Shop.GetItemsResponse>(
                service, METHODID_GET_ITEMS)))
        .addMethod(
          getBuyPrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.BuyPrefixRequest,
              com.tron.bridge.Prefix.BuyPrefixResponse>(
                service, METHODID_BUY_PREFIX)))
        .addMethod(
          getSelectPrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.SelectPrefixRequest,
              com.tron.bridge.Prefix.SelectPrefixResponse>(
                service, METHODID_SELECT_PREFIX)))
        .addMethod(
          getGetAllPrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.GetAllPrefixRequest,
              com.tron.bridge.Prefix.GetAllPrefixResponse>(
                service, METHODID_GET_ALL_PREFIX)))
        .addMethod(
          getGetOwnedPrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.GetOwnedPrefixRequest,
              com.tron.bridge.Prefix.GetOwnedPrefixResponse>(
                service, METHODID_GET_OWNED_PREFIX)))
        .addMethod(
          getGetCurrentPrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.GetCurrentPrefixRequest,
              com.tron.bridge.Prefix.GetCurrentPrefixResponse>(
                service, METHODID_GET_CURRENT_PREFIX)))
        .addMethod(
          getCreatePrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.CreatePrefixRequest,
              com.tron.bridge.Prefix.CreatePrefixResponse>(
                service, METHODID_CREATE_PREFIX)))
        .addMethod(
          getDeletePrefixMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Prefix.DeletePrefixRequest,
              com.tron.bridge.Prefix.DeletePrefixResponse>(
                service, METHODID_DELETE_PREFIX)))
        .addMethod(
          getPlayerDeathMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Player.PlayerDeathRequest,
              com.tron.bridge.Player.PlayerDeathResponse>(
                service, METHODID_PLAYER_DEATH)))
        .addMethod(
          getPlayerKillMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Player.PlayerKillRequest,
              com.tron.bridge.Player.PlayerKillResponse>(
                service, METHODID_PLAYER_KILL)))
        .addMethod(
          getPlayerPlaceBlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Player.PlayerPlaceBlockRequest,
              com.tron.bridge.Player.PlayerPlaceBlockResponse>(
                service, METHODID_PLAYER_PLACE_BLOCK)))
        .addMethod(
          getPlayerBreakBlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Player.PlayerBreakBlockRequest,
              com.tron.bridge.Player.PlayerBreakBlockResponse>(
                service, METHODID_PLAYER_BREAK_BLOCK)))
        .addMethod(
          getProxyStartupMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.ProxyStartupRequest,
              com.tron.bridge.Server.ProxyStartupResponse>(
                service, METHODID_PROXY_STARTUP)))
        .addMethod(
          getProxyShutdownMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.ProxyShutdownRequest,
              com.tron.bridge.Server.ProxyShutdownResponse>(
                service, METHODID_PROXY_SHUTDOWN)))
        .addMethod(
          getSurvivalStartupMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.SurvivalStartupRequest,
              com.tron.bridge.Server.SurvivalStartupResponse>(
                service, METHODID_SURVIVAL_STARTUP)))
        .addMethod(
          getSurvivalShutdownMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.SurvivalShutdownRequest,
              com.tron.bridge.Server.SurvivalShutdownResponse>(
                service, METHODID_SURVIVAL_SHUTDOWN)))
        .addMethod(
          getLobbyStartupMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.LobbyStartupRequest,
              com.tron.bridge.Server.LobbyStartupResponse>(
                service, METHODID_LOBBY_STARTUP)))
        .addMethod(
          getLobbyShutdownMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Server.LobbyShutdownRequest,
              com.tron.bridge.Server.LobbyShutdownResponse>(
                service, METHODID_LOBBY_SHUTDOWN)))
        .addMethod(
          getServerSendMessageMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.tron.bridge.Server.ServerSubscribeRequest,
              com.tron.bridge.Server.ServerSendMessageResponse>(
                service, METHODID_SERVER_SEND_MESSAGE)))
        .addMethod(
          getServerSendTitleMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.tron.bridge.Server.ServerSubscribeRequest,
              com.tron.bridge.Server.ServerSendTitleResponse>(
                service, METHODID_SERVER_SEND_TITLE)))
        .addMethod(
          getMessageMethod(),
          io.grpc.stub.ServerCalls.asyncBidiStreamingCall(
            new MethodHandlers<
              com.tron.bridge.Server.MessageRequest,
              com.tron.bridge.Server.MessageResponse>(
                service, METHODID_MESSAGE)))
        .addMethod(
          getReportPlayerMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.tron.bridge.Security.ReportPlayerRequest,
              com.tron.bridge.Security.ReportPlayerResponse>(
                service, METHODID_REPORT_PLAYER)))
        .build();
  }

  private static abstract class BridgeBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    BridgeBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.tron.bridge.BridgeProto.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("Bridge");
    }
  }

  private static final class BridgeFileDescriptorSupplier
      extends BridgeBaseDescriptorSupplier {
    BridgeFileDescriptorSupplier() {}
  }

  private static final class BridgeMethodDescriptorSupplier
      extends BridgeBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    BridgeMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (BridgeGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new BridgeFileDescriptorSupplier())
              .addMethod(getPlayerJoinMethod())
              .addMethod(getPlayerLeaveMethod())
              .addMethod(getGetBalanceMethod())
              .addMethod(getTransferBalanceMethod())
              .addMethod(getOverallLeaderboardMethod())
              .addMethod(getCoinsLeaderboardMethod())
              .addMethod(getTeamsLeaderboardMethod())
              .addMethod(getKdaLeaderboardMethod())
              .addMethod(getDeathsLeaderboardMethod())
              .addMethod(getKillsLeaderboardMethod())
              .addMethod(getGetFriendsMethod())
              .addMethod(getListFriendsMethod())
              .addMethod(getSendFriendRequestMethod())
              .addMethod(getAcceptFriendRequestMethod())
              .addMethod(getRejectFriendRequestMethod())
              .addMethod(getGetFriendRequestsMethod())
              .addMethod(getListFriendRequestsMethod())
              .addMethod(getRemoveFriendMethod())
              .addMethod(getCreateTeamMethod())
              .addMethod(getDeleteTeamMethod())
              .addMethod(getLeaveTeamMethod())
              .addMethod(getJoinTeamMethod())
              .addMethod(getSendTeamInviteMethod())
              .addMethod(getAcceptTeamInviteMethod())
              .addMethod(getRejectTeamInviteMethod())
              .addMethod(getGetTeamMembersMethod())
              .addMethod(getRemoveTeamMemberMethod())
              .addMethod(getPromoteTeamMemberMethod())
              .addMethod(getGetOpenTeamsMethod())
              .addMethod(getBuyItemMethod())
              .addMethod(getSellItemMethod())
              .addMethod(getGetItemsMethod())
              .addMethod(getBuyPrefixMethod())
              .addMethod(getSelectPrefixMethod())
              .addMethod(getGetAllPrefixMethod())
              .addMethod(getGetOwnedPrefixMethod())
              .addMethod(getGetCurrentPrefixMethod())
              .addMethod(getCreatePrefixMethod())
              .addMethod(getDeletePrefixMethod())
              .addMethod(getPlayerDeathMethod())
              .addMethod(getPlayerKillMethod())
              .addMethod(getPlayerPlaceBlockMethod())
              .addMethod(getPlayerBreakBlockMethod())
              .addMethod(getProxyStartupMethod())
              .addMethod(getProxyShutdownMethod())
              .addMethod(getSurvivalStartupMethod())
              .addMethod(getSurvivalShutdownMethod())
              .addMethod(getLobbyStartupMethod())
              .addMethod(getLobbyShutdownMethod())
              .addMethod(getServerSendMessageMethod())
              .addMethod(getServerSendTitleMethod())
              .addMethod(getMessageMethod())
              .addMethod(getReportPlayerMethod())
              .build();
        }
      }
    }
    return result;
  }
}
