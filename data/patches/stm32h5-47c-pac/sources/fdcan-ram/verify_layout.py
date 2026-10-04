"""Evaluate exact ST layout macros and instance-selection prefix, without MMIO."""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
SOURCES = ROOT / 'data/patches/stm32h5-47c/sources'
lock = json.loads((SOURCES.parent / 'sources.json').read_text(encoding='utf-8'))
hal_path = OUT / 'stm32h5xx_hal_fdcan.c'
hal_raw = hal_path.read_bytes()
assert hashlib.sha256(hal_raw).hexdigest() == 'a430429a327511cf96203dcebd45a1ab8491d1f09ecb19027333b718c3f35b53'
hal = hal_raw.decode('utf-8')
without_comments = re.sub(r'/\*.*?\*/', '', hal, flags=re.S)
ram_macros = '\n'.join(line for line in without_comments.splitlines() if line.startswith('#define SRAMCAN_'))
marker = 'static void FDCAN_CalcultateRamBlockAddresses(FDCAN_HandleTypeDef *hfdcan)\n{'
start = hal.index(marker)
prefix = hal[start + len(marker):hal.index('  /* Standard filter list start address */', start)]
# Exact original prefix performs only arithmetic and compares peripheral pointer
# values. Stop before any register dereference or RAM-clearing loop.
fields = ['SRAMCAN_FLSSA','SRAMCAN_FLESA','SRAMCAN_RF0SA','SRAMCAN_RF1SA','SRAMCAN_TEFSA','SRAMCAN_TFQSA','SRAMCAN_SIZE']
results = {'hal_source_sha256':hashlib.sha256(hal_raw).hexdigest(),'cmsis_commit':lock['revisions']['cmsis_h5'],'cases':[]}
for chip in ('stm32h543xx','stm32h553xx'):
    raw = (SOURCES / (chip + '.h')).read_bytes()
    expected = lock['files'][chip + '.h']['sha256']
    assert hashlib.sha256(raw).hexdigest() == expected
    header = re.sub(r'/\*.*?\*/','',raw.decode('utf-8'),flags=re.S)
    lines = header.splitlines()
    for security in ('NS','S'):
        names = [f'{name}_{security}' for name in ('PERIPH_BASE','APB1PERIPH_BASE','SRAMCAN_BASE','FDCAN1_BASE','FDCAN2_BASE','FDCAN1','FDCAN2')]
        definitions = []
        for name in names:
            matches=[line for line in lines if re.match(r'#define\s+' + name + r'\s+', line)]
            assert len(matches)==1,(chip,name)
            definitions.extend(matches)
        for name in ('SRAMCAN_BASE','FDCAN1','FDCAN2'):
            matches=[line for line in lines if re.match(r'#define\s+'+name+r'\s+'+name+'_'+security+r'\s*$',line)]
            assert len(matches)==1,(chip,name,security)
            definitions.extend(matches)
        assert not re.search(r'^#define\s+FDCAN3\s',header,flags=re.M)
        c = '#include <stdio.h>\n#include <stdint.h>\ntypedef struct {uint32_t dummy;} FDCAN_GlobalTypeDef;\n'
        c += '\n'.join(definitions)+'\n'+ram_macros+'\n'
        c += 'typedef struct {FDCAN_GlobalTypeDef *Instance;} FDCAN_HandleTypeDef;\n'
        c += 'static uint32_t selected_base(FDCAN_HandleTypeDef *hfdcan) {'+prefix+'\n (void)sizeof(RAMcounter); return SramCanInstanceBase; }\n'
        c += 'int main(void) { FDCAN_HandleTypeDef first={FDCAN1},second={FDCAN2};\n'
        c += 'printf("%u %u",selected_base(&first),selected_base(&second));\n'
        for field in fields:c += f'printf(" %u",(unsigned){field});\n'
        c += 'return 0;}\n'
        stem=OUT/(chip+'-'+security.lower())
        source=stem.with_suffix('.c'); binary=stem.with_suffix('.exe')
        source.write_text(c,encoding='utf-8')
        command=['gcc','-std=c11','-Wall','-Wextra','-Werror',str(source),'-o',str(binary)]
        compile=subprocess.run(command,capture_output=True,text=True)
        assert compile.returncode==0,compile.stderr
        output=subprocess.check_output([str(binary)],text=True)
        values=list(map(int,output.split()))
        expected_base=0x4000ac00 if security=='NS' else 0x5000ac00
        assert values == [expected_base,expected_base+0x350,0,0x70,0xb0,0x188,0x260,0x278,0x350],values
        results['cases'].append({'chip':chip,'security_alias':security,'cmsis_sha256':expected,'compile_command':command,'values':dict(zip(['FDCANRAM1','FDCANRAM2']+fields,values))})
(OUT/'verified-layout.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
print(json.dumps(results,indent=2))
