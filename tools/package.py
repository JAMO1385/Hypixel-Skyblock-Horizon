from pathlib import Path
import json
import hashlib
import zipfile
import tomllib

ROOT=Path(__file__).resolve().parents[1]

def package():
    compat=json.loads((ROOT/'compatibility.json').read_text())
    bundle=ROOT/'mixins/horizon_skyblock'
    config=tomllib.loads((bundle/'horizon-mixin.toml').read_text())
    manifest={'schema':1,'id':config['mod']['id'],'version':config['mod']['version'],'game':compat['game'],'horizon':compat['horizon_revision'],'mixins':compat['horizon_mixins_revision']}
    # Intentionally pinned: reading the local identity uses Game::own_uuid, which is not a stable helper.
    assert config['compatibility']['horizon_revision']==manifest['horizon']
    files={'horizon-native.json':(json.dumps(manifest,indent=2)+'\n').encode()}
    for p in sorted(bundle.rglob('*')):
        if p.is_file() and p.suffix in ('.toml','.rs') and 'target' not in p.relative_to(bundle).parts:
            files['mixins/'+manifest['id']+'/'+p.relative_to(bundle).as_posix()]=p.read_bytes()
    files['mixins/'+manifest['id']+'/LICENSE']=(ROOT/'LICENSE').read_bytes()
    out=ROOT/'dist'/f"{manifest['id']}-{manifest['version']}.hnmod"
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out,'w',compression=zipfile.ZIP_DEFLATED) as z:
        for name,data in sorted(files.items()):
            info=zipfile.ZipInfo(name,(2026,11,13,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=0o100644<<16;z.writestr(info,data)
    with zipfile.ZipFile(out) as z:
        assert z.testzip() is None
        assert len(z.namelist())==len(set(z.namelist()))
        assert json.loads(z.read('horizon-native.json'))==manifest
        assert all('..' not in Path(n).parts for n in z.namelist())
    checksum=hashlib.sha256(out.read_bytes()).hexdigest()
    (out.parent/'SHA256SUMS').write_text(f'{checksum}  {out.name}\n')
    print(json.dumps({'file':str(out),'size':out.stat().st_size,'sha256':checksum,'entries':len(files),'compatibility':'pinned'},indent=2))
    return out

if __name__=='__main__':package()
