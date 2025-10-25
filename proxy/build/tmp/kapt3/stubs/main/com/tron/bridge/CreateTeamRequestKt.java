package com.tron.bridge;

/**
 * Protobuf type `bridge.CreateTeamRequest`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/CreateTeamRequestKt;", "", "()V", "Dsl", "proxy"})
public final class CreateTeamRequestKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.CreateTeamRequestKt INSTANCE = null;
    
    private CreateTeamRequestKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u00000\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0010\u000e\n\u0002\b\u0005\n\u0002\u0010\u000b\n\u0002\b\f\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0005\b\u0007\u0018\u0000 \u001f2\u00020\u0001:\u0001\u001fB\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u0018\u001a\u00020\u0019H\u0001J\u0006\u0010\u001a\u001a\u00020\u001bJ\u0006\u0010\u001c\u001a\u00020\u001bJ\u0006\u0010\u001d\u001a\u00020\u001bJ\u0006\u0010\u001e\u001a\u00020\u001bR\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R$\u0010\u0007\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR$\u0010\r\u001a\u00020\f2\u0006\u0010\u0005\u001a\u00020\f8G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u000e\u0010\u000f\"\u0004\b\u0010\u0010\u0011R$\u0010\u0012\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0013\u0010\t\"\u0004\b\u0014\u0010\u000bR$\u0010\u0015\u001a\u00020\u00062\u0006\u0010\u0005\u001a\u00020\u00068G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0016\u0010\t\"\u0004\b\u0017\u0010\u000b\u00a8\u0006 "}, d2 = {"Lcom/tron/bridge/CreateTeamRequestKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Teams$CreateTeamRequest$Builder;", "(Lcom/tron/bridge/Teams$CreateTeamRequest$Builder;)V", "value", "", "color", "getColor", "()Ljava/lang/String;", "setColor", "(Ljava/lang/String;)V", "", "open", "getOpen", "()Z", "setOpen", "(Z)V", "team", "getTeam", "setTeam", "username", "getUsername", "setUsername", "_build", "Lcom/tron/bridge/Teams$CreateTeamRequest;", "clearColor", "", "clearOpen", "clearTeam", "clearUsername", "Companion", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Teams.CreateTeamRequest.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.CreateTeamRequestKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Teams.CreateTeamRequest.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getUsername")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getUsername() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setUsername")
        public final void setUsername(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string username = 1;`
         */
        public final void clearUsername() {
        }
        
        @kotlin.jvm.JvmName(name = "getTeam")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getTeam() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setTeam")
        public final void setTeam(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string team = 2;`
         */
        public final void clearTeam() {
        }
        
        @kotlin.jvm.JvmName(name = "getColor")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getColor() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setColor")
        public final void setColor(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string color = 3;`
         */
        public final void clearColor() {
        }
        
        @kotlin.jvm.JvmName(name = "getOpen")
        public final boolean getOpen() {
            return false;
        }
        
        @kotlin.jvm.JvmName(name = "setOpen")
        public final void setOpen(boolean value) {
        }
        
        /**
         * `bool open = 4;`
         */
        public final void clearOpen() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/CreateTeamRequestKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/CreateTeamRequestKt$Dsl;", "builder", "Lcom/tron/bridge/Teams$CreateTeamRequest$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
    }
}