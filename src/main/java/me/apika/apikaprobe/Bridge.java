package me.apika.apikaprobe;

import java.util.function.Function;

import net.fabricmc.fabric.api.event.Event;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents.ModifyEntries;
import net.minecraft.entity.Entity;
import net.minecraft.entity.LightningEntity;
import net.minecraft.entity.passive.WolfEntity;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.item.Item;
import net.minecraft.text.Text;
import net.minecraft.util.ActionResult;
import net.minecraft.util.Hand;
import net.minecraft.util.hit.HitResult;
import net.minecraft.util.math.Direction;
import net.minecraft.util.math.Vec3d;
import net.minecraft.world.World;

class SerjioItem extends Item {
  public SerjioItem(Item.Settings settings) {
    super(settings);
  }

  @Override
  public native ActionResult use(World world, PlayerEntity user, Hand hand);

  // @Override public ActionResult use(World world, PlayerEntity user, Hand hand) {
  //   Text
  // };
}

class RustBridge {
  public static <T extends Item> Function<Item.Settings, Item> itemFactory(Class<T> item) {
    return (settings) -> {
      try {
        return item.getDeclaredConstructor(Item.Settings.class).newInstance(settings);
      } catch (Exception e) {
        ExampleMod.LOGGER.info("======================================= Cannot create new instance ");
        return null;
      }
    };
  }

  public static void registerGroupEvent(Event<ModifyEntries> event, Item item) {
    event.register((entries) -> entries.add(item));
  }

  public static Item SERJIO = null;

  static {
    System.loadLibrary("rust_mod");
  }

  public static native void main();
}
