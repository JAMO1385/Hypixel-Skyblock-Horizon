"""Prepare an isolated exact-source hook workspace with fewer indexed crates.
Only workspace membership and inherited package metadata change. All .rs files
are untouched copies of the pinned upstream. Use to bound mixins memory.
"""
from pathlib import Path
import os
import re
import shutil
import sys
import tomllib

source=Path(sys.argv[1]).resolve()
out=Path(os.environ['TMPDIR'])/'horizon-hook-source'
if out.exists():raise SystemExit('Hook source already exists; use the existing copy')
shutil.copytree(source,out,ignore=shutil.ignore_patterns('target','.git','.local'))
root=out/'Cargo.toml'
s=root.read_text()
s=re.sub(r'members = \[.*?\]\nresolver', 'members = ["crates/horizon-client"]\nresolver',s,flags=re.S)
data=tomllib.loads(s)
excludes=list(data['workspace'].get('exclude',[]))
excludes.extend(str(p.parent.relative_to(out)) for p in (out/'crates').glob('*/Cargo.toml') if p.parent.name!='horizon-client')
import json
s=re.sub(r'exclude = \[.*?\]', 'exclude = '+json.dumps(sorted(set(excludes))),s,count=1,flags=re.S)
root.write_text(s)
(out/'.horizon-revision').write_text('9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0')
for p in (out/'crates').glob('*/Cargo.toml'):
    if p.parent.name=='horizon-client':continue
    text=p.read_text()
    for key,value in [('edition','"2024"'),('license','"MIT"'),('rust-version','"1.95"')]:
        text=text.replace(f'{key}.workspace = true',f'{key} = {value}')
    text=text.replace('[lints]\nworkspace = true','[lints]')
    p.write_text(text)
print(out)
