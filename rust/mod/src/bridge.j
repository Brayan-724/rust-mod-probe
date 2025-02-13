import java.util.function.Function;
import net.fabricmc.fabric.api.event.Event;
import net.fabricmc.fabric.api.itemgroup.v1.ItemGroupEvents.ModifyEntries;
import net.minecraft.item.Item;

class $CLASS_NAME {
  public static <T extends Item> Function<Item.Settings, Item> itemFactory(Class<T> item) {
    return (settings) -> {
      try {
        return item.getDeclaredConstructor(Item.Settings.class).newInstance(settings);
      } catch (Exception e) {
        ExampleMod.LOGGER.error("======================================= Cannot create new instance ");
        return null;
      }
    };
  }

  public static void registerGroupEvent(Event<ModifyEntries> event, Item item) {
    event.register((entries) -> entries.add(item));
  }

  /* ROSTTASSE-CONTENT */
}

//* vim set: filetype=java
