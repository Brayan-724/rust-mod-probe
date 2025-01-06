package com.apika_probe_1;

import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents;
import net.minecraft.item.Item;
import net.minecraft.item.Items;
import net.minecraft.item.ItemGroups;
import net.minecraft.registry.RegistryKey;

public class ModItems {
  public static final Item SUSPICIOUS_SUBSTANCE = RustBridge.register(
      RustBridge.itemId(),
      new Item.Settings());

  public static void initialize() {
    ItemGroupEvents.modifyEntriesEvent(ItemGroups.INGREDIENTS)
        .register((itemGroup) -> itemGroup.add(ModItems.SUSPICIOUS_SUBSTANCE));
  }
}
