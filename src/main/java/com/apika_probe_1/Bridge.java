package com.apika_probe_1;

import java.util.function.Function;

import net.minecraft.item.Item;

class RustBridge {
  static {
    System.loadLibrary("rust_bridge");
  }

  public static native String itemId();

  public static native Item register(String id, Item.Settings settings);

  public static Function<Item.Settings, Item> getItemNew() {
    return Item::new;
  }
}
