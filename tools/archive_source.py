from pathlib import Path
import hashlib
import zipfile

root=Path(__file__).resolve().parents[1]
out=root/'dist'/'horizon_skyblock-0.1.0-source.zip'
exclude={'.git','target','dist','.superpowers','__pycache__'}
with zipfile.ZipFile(out,'w',compression=zipfile.ZIP_DEFLATED) as z:
    for p in sorted(root.rglob('*')):
        if not p.is_file():
            continue
        rel=p.relative_to(root)
        if any(part in exclude for part in rel.parts) or p.suffix=='.pyc':
            continue
        info=zipfile.ZipInfo(('horizon-skyblock/'+rel.as_posix()),(2026,11,13,0,0,0))
        info.compress_type=zipfile.ZIP_DEFLATED
        info.external_attr=0o100644<<16
        z.writestr(info,p.read_bytes())
with zipfile.ZipFile(out) as z:
    assert z.testzip() is None
    assert len(z.namelist())==len(set(z.namelist()))
sha=hashlib.sha256(out.read_bytes()).hexdigest()
(root/'dist'/'SHA256SUMS').write_text(
    f'{hashlib.sha256((root/"dist"/"horizon_skyblock-0.1.0.hnmod").read_bytes()).hexdigest()}  horizon_skyblock-0.1.0.hnmod\n'
    +f'{sha}  {out.name}\n'
)
print(f'{out}\n{out.stat().st_size}\n{sha}\n{len(zipfile.ZipFile(out).namelist())} entries')
