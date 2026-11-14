from pathlib import Path
import json

root=Path(__file__).resolve().parents[1]
bundle=root/'mixins/horizon_skyblock'
revision=json.loads((root/'compatibility.json').read_text())['horizon_revision']
lines=['[mod]','id = "horizon_skyblock"','version = "0.1.0"','priority = 100','','[compatibility]',f'horizon_revision = "{revision}"','','[[crate]]','id = "core"','path = "core"','','[[module]]','id = "runtime"','file = "crates/horizon-client/src/game.rs"','name = "horizon_skyblock"','source = "runtime.rs"','']
game=('game',"&mut crate::game::Game<'_>",'&mut *self')
readonly=('game',"&crate::game::Game<'_>",'&*self')
hooks=[
('packet','client_packet',[game,('packet','&horizon_client::mod_api::ClientboundPlayPacket','packet')]),
('tick','tick',[game]),
('hud','hud',[game,('batch','&mut horizon_client::hud::HudBatch','&mut *batch'),('corners','&mut horizon_client::extras_hud::Corners','&mut *corners')]),
('key','key',[game,('key','&str','key'),('pressed','bool','pressed')]),
('command','command',[game,('command','&str','command'),('ran','&mut bool','&mut *ran')]),
('list','mods',[('mods','&mut Vec<crate::game::native_hooks::NativeMod>','&mut *mods')]),
('settings','settings',[game,('mod_id','&str','mod_id'),('buttons','&mut Vec<String>','&mut *buttons')]),
('click','settings_click',[game,('mod_id','&str','mod_id'),('button','usize','button')]),
('disconnect','client_disconnecting',[readonly]),
]
for name,hook,bindings in hooks:
    lines+=['[[injection]]',f'id = "{name}"','kind = "HEAD"',f'target = "horizon_client::game::native_hooks::Game<\'_>::native_{hook}"',f'helper = "{name}.rs"','bindings = [']
    lines += ['  { name = '+json.dumps(n)+', ty = '+json.dumps(t)+', value = '+json.dumps(v)+' },' for n,t,v in bindings]
    lines += [']','']
(bundle/'horizon-mixin.toml').write_text('\n'.join(lines))
print(bundle/'horizon-mixin.toml')
