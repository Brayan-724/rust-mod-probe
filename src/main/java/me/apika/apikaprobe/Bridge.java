package me.apika.apikaprobe;

import java.util.function.Function;

import net.fabricmc.fabric.api.event.Event;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents.ModifyEntries;
import net.minecraft.item.Item;

class RustBridge {
  public static Function<Item.Settings, Item> getItemNew() {
    return Item::new;
  }

  public static void registerGroupEvent(Event<ModifyEntries> event, Item item) {
    event.register((entries) -> entries.add(item));
  }

  public static Item SUSPICIOUS_SUBSTANCE = null;

  static {
    System.loadLibrary("rust_bridge");
  }

  public static native void main();
}
