package com.tron.bridge;

/**
 * Protobuf type `bridge.Item`
 */
@kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0003\b\u00c6\u0002\u0018\u00002\u00020\u0001:\u0001\u0003B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0004"}, d2 = {"Lcom/tron/bridge/ItemKt;", "", "()V", "Dsl", "proxy"})
public final class ItemKt {
    @org.jetbrains.annotations.NotNull()
    public static final com.tron.bridge.ItemKt INSTANCE = null;
    
    private ItemKt() {
        super();
    }
    
    @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000H\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0018\u0002\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0002\u0010\u000e\n\u0002\u0018\u0002\n\u0002\b\b\n\u0002\u0018\u0002\n\u0002\b\u0005\n\u0002\u0010\b\n\u0002\b\u0006\n\u0002\u0018\u0002\n\u0000\n\u0002\u0010\u0002\n\u0002\b\u0005\n\u0002\u0010\u001c\n\u0002\b\f\b\u0007\u0018\u0000 02\u00020\u0001:\u000201B\u000f\b\u0002\u0012\u0006\u0010\u0002\u001a\u00020\u0003\u00a2\u0006\u0002\u0010\u0004J\b\u0010\u001d\u001a\u00020\u001eH\u0001J\u0006\u0010\u001f\u001a\u00020 J\u0006\u0010!\u001a\u00020 J%\u0010\"\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00062\u0006\u0010\u000b\u001a\u00020\u0007H\u0007\u00a2\u0006\u0002\b#J+\u0010$\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00062\f\u0010%\u001a\b\u0012\u0004\u0012\u00020\u00070&H\u0007\u00a2\u0006\u0002\b\'J\u001d\u0010(\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u0006H\u0007\u00a2\u0006\u0002\b)J&\u0010*\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00062\u0006\u0010\u000b\u001a\u00020\u0007H\u0087\n\u00a2\u0006\u0002\b+J,\u0010*\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00062\f\u0010%\u001a\b\u0012\u0004\u0012\u00020\u00070&H\u0087\n\u00a2\u0006\u0002\b,J.\u0010-\u001a\u00020 *\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00062\u0006\u0010.\u001a\u00020\u00172\u0006\u0010\u000b\u001a\u00020\u0007H\u0087\u0002\u00a2\u0006\u0002\b/R\u000e\u0010\u0002\u001a\u00020\u0003X\u0082\u0004\u00a2\u0006\u0002\n\u0000R\u001d\u0010\u0005\u001a\u000e\u0012\u0004\u0012\u00020\u0007\u0012\u0004\u0012\u00020\b0\u00068F\u00a2\u0006\u0006\u001a\u0004\b\t\u0010\nR$\u0010\f\u001a\u00020\u00072\u0006\u0010\u000b\u001a\u00020\u00078G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\r\u0010\u000e\"\u0004\b\u000f\u0010\u0010R$\u0010\u0012\u001a\u00020\u00112\u0006\u0010\u000b\u001a\u00020\u00118G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0013\u0010\u0014\"\u0004\b\u0015\u0010\u0016R$\u0010\u0018\u001a\u00020\u00172\u0006\u0010\u000b\u001a\u00020\u00178G@GX\u0086\u000e\u00a2\u0006\f\u001a\u0004\b\u0019\u0010\u001a\"\u0004\b\u001b\u0010\u001c\u00a8\u00062"}, d2 = {"Lcom/tron/bridge/ItemKt$Dsl;", "", "_builder", "Lcom/tron/bridge/Common$Item$Builder;", "(Lcom/tron/bridge/Common$Item$Builder;)V", "enchantments", "Lcom/google/protobuf/kotlin/DslList;", "", "Lcom/tron/bridge/ItemKt$Dsl$EnchantmentsProxy;", "getEnchantments", "()Lcom/google/protobuf/kotlin/DslList;", "value", "id", "getId", "()Ljava/lang/String;", "setId", "(Ljava/lang/String;)V", "Lcom/tron/bridge/Common$ItemType;", "itemType", "getItemType", "()Lcom/tron/bridge/Common$ItemType;", "setItemType", "(Lcom/tron/bridge/Common$ItemType;)V", "", "itemTypeValue", "getItemTypeValue", "()I", "setItemTypeValue", "(I)V", "_build", "Lcom/tron/bridge/Common$Item;", "clearId", "", "clearItemType", "add", "addEnchantments", "addAll", "values", "", "addAllEnchantments", "clear", "clearEnchantments", "plusAssign", "plusAssignEnchantments", "plusAssignAllEnchantments", "set", "index", "setEnchantments", "Companion", "EnchantmentsProxy", "proxy"})
    @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
    @com.google.protobuf.kotlin.ProtoDslMarker()
    public static final class Dsl {
        @org.jetbrains.annotations.NotNull()
        private final com.tron.bridge.Common.Item.Builder _builder = null;
        @org.jetbrains.annotations.NotNull()
        public static final com.tron.bridge.ItemKt.Dsl.Companion Companion = null;
        
        private Dsl(com.tron.bridge.Common.Item.Builder _builder) {
            super();
        }
        
        @kotlin.jvm.JvmName(name = "getId")
        @org.jetbrains.annotations.NotNull()
        public final java.lang.String getId() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setId")
        public final void setId(@org.jetbrains.annotations.NotNull()
        java.lang.String value) {
        }
        
        /**
         * `string id = 1;`
         */
        public final void clearId() {
        }
        
        @kotlin.jvm.JvmName(name = "getItemType")
        @org.jetbrains.annotations.NotNull()
        public final com.tron.bridge.Common.ItemType getItemType() {
            return null;
        }
        
        @kotlin.jvm.JvmName(name = "setItemType")
        public final void setItemType(@org.jetbrains.annotations.NotNull()
        com.tron.bridge.Common.ItemType value) {
        }
        
        @kotlin.jvm.JvmName(name = "getItemTypeValue")
        public final int getItemTypeValue() {
            return 0;
        }
        
        @kotlin.jvm.JvmName(name = "setItemTypeValue")
        public final void setItemTypeValue(int value) {
        }
        
        /**
         * `.bridge.ItemType item_type = 2;`
         */
        public final void clearItemType() {
        }
        
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\u0018\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0002\b\u0002\n\u0002\u0018\u0002\n\u0000\n\u0002\u0018\u0002\n\u0000\b\u0086\u0003\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002J\u0010\u0010\u0003\u001a\u00020\u00042\u0006\u0010\u0005\u001a\u00020\u0006H\u0001\u00a8\u0006\u0007"}, d2 = {"Lcom/tron/bridge/ItemKt$Dsl$Companion;", "", "()V", "_create", "Lcom/tron/bridge/ItemKt$Dsl;", "builder", "Lcom/tron/bridge/Common$Item$Builder;", "proxy"})
        public static final class Companion {
            
            private Companion() {
                super();
            }
        }
        
        /**
         * An uninstantiable, behaviorless type to represent the field in
         * generics.
         */
        @kotlin.Metadata(mv = {1, 9, 0}, k = 1, xi = 48, d1 = {"\u0000\f\n\u0002\u0018\u0002\n\u0002\u0018\u0002\n\u0002\b\u0002\u0018\u00002\u00020\u0001B\u0007\b\u0002\u00a2\u0006\u0002\u0010\u0002\u00a8\u0006\u0003"}, d2 = {"Lcom/tron/bridge/ItemKt$Dsl$EnchantmentsProxy;", "Lcom/google/protobuf/kotlin/DslProxy;", "()V", "proxy"})
        @kotlin.OptIn(markerClass = {com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode.class})
        public static final class EnchantmentsProxy extends com.google.protobuf.kotlin.DslProxy {
            
            private EnchantmentsProxy() {
                super();
            }
        }
    }
}