fn skyblock_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::horizon_skyblock as m;
    mods.push(crate::game::native_hooks::NativeMod{id:m::ID.into(),title:format!("{} {}",m::NAME,m::VERSION),description:"Hypixel SkyBlock HUD, Slayer and dungeon trackers, reminders, local waypoints and lobby shortcuts. /hs help".into(),keys:m::KEYS.iter().map(|(n,l,k)|[(*n).into(),(*l).into(),(*k).into()]).collect()});
}
