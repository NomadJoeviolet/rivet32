# H5 RTC / low-power candidate overlay

This package reproduces the exact candidate checked on 40 ARM HAL selections and
eight linked RTC/executor firmware representatives. It is not integrated into
the main framework yet. Hardware, retained backup-domain reset, digital LSE and
USB warm-PHY limitations remain open.

The HAL delta changes only build.rs, src/low_power.rs and src/rtc/low_power.rs.
The PAC delta changes RTC metadata/selection for 38 exact H5 parts and adds six
native RTC v3 interface variants (register and peripheral modules). Existing
Cargo feature definitions, physical register offsets and unrelated drivers are
preserved. The archived chip JSONs are the RTC generator inputs; the original
chip packages remain the physical input baseline for this overlay.

From the repository root, before main integration:

```powershell
python target/h5-rtc-interface-v1/portable-v1/replay.py --sources vendor --output target/h5-rtc-portable-replayed-v1
```

The source directory must contain embassy-stm32 and stm32-metapac with the exact
baseline inventories. Output must not exist. To restore the baseline into a new
directory, use --restore and pass the replayed output as --sources. Existing
sources are never overwritten. A mixed or modified tree is rejected.

changes.zip stores only changed preimages/postimages and added files.
generator-inputs.zip retains all 457 generator input files, including 38 exact
chips and six new RTC schemas; the unchanged generator revision is recorded in
overlay.json. references.zip retains ST RTC sources, CMSIS headers, checked
constants, license texts and the original preparation scripts as history.
Those historical scripts preserve their original workspace paths; replay.py is
the standalone portable replay entry point and needs only Python's standard
library, this package and its exact baseline source tree.

Every archive member and package file is hash-bound. The two baseline manifests
and complete before/after source inventories allow verification without another
full vendor archive. This package does not claim physical RTC wakeup, STOP
recovery, USB transfer or H7R/S CAN qualification.
