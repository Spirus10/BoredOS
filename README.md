# BoredOS
As in: Bored? Work on your OS
---
Linking the kernel to the bootloader requires a build tool called `bootimage`
> ```bash 
> cargo install bootimage
> ```
bootimage requires the component `llvm-tools-preview`
> ```bash
> rustup component add llvm-tools-preview
> ```
To build, run:
> ```bash
> cargo bootimage
> ```
