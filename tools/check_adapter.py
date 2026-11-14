"""Type-check native helpers against real Horizon packet/Text types.

Only Game/window/render boundaries are stand-ins; this is NOT a full game check
or an in-game test. Run with the pinned pristine Horizon source directory.
"""
from pathlib import Path
import json
import os
import subprocess
import sys

repo=Path(__file__).resolve().parents[1]
source=Path(sys.argv[1]).resolve()
out=Path(os.environ['TMPDIR'])/'skyblock-adapter-check'
out.mkdir(exist_ok=True)
core=repo/'mixins/horizon_skyblock/core'
cargo=f'''[package]
name = "skyblock-adapter-check"
version = "0.1.0"
edition = "2024"
[workspace]
[dependencies]
horizon-protocol = {{ path = {json.dumps(str(source/'crates/horizon-protocol'))} }}
horizon-world = {{ path = {json.dumps(str(source/'crates/horizon-world'))} }}
horizon-client-core = {{ path = {json.dumps(str(source/'crates/horizon-client-core'))} }}
horizon_skyblock_core = {{ path = {json.dumps(str(core))} }}
[patch.crates-io]
libmimalloc-sys = {{ path = {json.dumps(str(source/'third_party/libmimalloc-sys'))} }}
'''
(out/'Cargo.toml').write_text(cargo)
code='''#![allow(dead_code)]
extern crate self as horizon_client;
pub use horizon_client_core::{scoreboard,text};
pub mod mod_api {pub use horizon_protocol::packets::play::Clientbound as ClientboundPlayPacket;}
pub mod hud {pub struct HudBatch;}
pub mod extras {#[derive(Clone,Copy)] pub enum Corner {TopLeft,TopRight,BottomLeft,BottomRight}}
pub mod extras_hud {pub struct Corners;impl Corners{pub fn place(&mut self,_:crate::extras::Corner,_:f32,_:f32)->(f32,f32){(0.0,0.0)}}}
pub mod game {
    pub struct Game<'a>{pub marker:std::marker::PhantomData<&'a ()>}
    pub mod native_hooks {pub struct NativeMod{pub id:String,pub title:String,pub description:String,pub keys:Vec<[String;3]>}}
    pub struct Player{pub x:f64,pub y:f64,pub z:f64,pub dimension:String}
    pub struct State{pub server_address:Option<String>,pub gui_size:[f32;2]}
    impl Game<'_>{
        pub fn native_setting(&self,_:&str,_:&str)->Option<String>{None}
        pub fn native_set_setting(&self,_:&str,_:&str,_:&str){}
        pub fn native_client_state(&self)->Option<State>{None}
        pub fn native_player(&self)->Option<Player>{None}
        pub fn own_uuid(&self)->u128{0}
        pub fn native_window(&self)->bool{true}
        pub fn native_on(&self,_:&str)->bool{true}
        pub fn native_playing(&self)->bool{true}
        pub fn native_chat(&mut self,_:&str){}
        pub fn native_send_chat(&mut self,_:&str)->Result<(),String>{Ok(())}
        pub fn native_command(&mut self,_:&str,_:&mut bool){}
        pub fn native_mod_key(&self,_:&str,_:&str,d:&str)->String{d.into()}
        pub fn native_frame_rate(&self)->(u32,u32,u32){(0,0,0)}
        pub fn native_mapping(&mut self,_:&str)->Option<(bool,String)>{None}
        pub fn native_clicks(&mut self)->(u32,u32){(0,0)}
        pub fn native_corner_shown(&self,_:crate::extras::Corner)->bool{true}
        pub fn native_text_width(&self,_:&str)->f32{0.0}
        pub fn native_fill(&self,_:&mut crate::hud::HudBatch,_:[f32;4],_:u32){}
        pub fn native_text(&self,_:&mut crate::hud::HudBatch,_:&str,_:[f32;2],_:u32,_:bool){}
    }
'''
bundle=repo/'mixins/horizon_skyblock'
code+=f'#[path={json.dumps(str(bundle/"runtime.rs"))}] pub mod horizon_skyblock;\n'
for name in ['packet','tick','hud','command','key','list','settings','click','disconnect']:
    code+=f'include!({json.dumps(str(bundle/(name+".rs")))});\n'
code+='}\n'
(out/'src').mkdir(exist_ok=True)
(out/'src/lib.rs').write_text(code)
subprocess.run(['cargo','check','--offline','--manifest-path',str(out/'Cargo.toml')],check=True)
print('Adapter type-check passed (real packet/Text crates; stand-in Game/render boundary).')
