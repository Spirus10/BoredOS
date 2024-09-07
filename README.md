# BoredOS
As in: Bored? Work on your OS
---

This project was developed as a learning exercise to deepen understanding of operating systems and kernel modules. It builds upon concepts from [Phillip Opperman's "Writing an OS in Rust"](https://os.phil-opp.com/) to explore and experiment with various aspects of OS and kernel module development. The primary goal was to gain hands-on experience and enhance knowledge in this domain.

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
