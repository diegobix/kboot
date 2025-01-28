# Kboot

### Build del proyecto

Primero se debe clonar el proyecto. Tras ello, debemos acceder al repositorio local.
El proyecto incluye un script para instalar las dependencias en sistemas Ubuntu, Debian y Fedora. Si tu sistema es otro, debes instalar las siguientes dependencias a mano:

- NASM
- GNU Make
- GNU Binutils
- QEMU system i386
- Rust nightly + Rustup + Cargo 
  
Si usas Ubuntu, Debian o Fedora, puedes ejecutar `setup.sh`:
```sh
chmod +x ./setup.sh
sudo ./setup.sh
```

Una vez tengas las dependencias, puedes compilar el proyecto usando GNU Make

```sh
make
```

También se puede ejecutar en máquina virtual con el siguiente comando:

```sh
make run
```

Debug con gdb:

```sh
make debug
```
