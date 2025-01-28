#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
  echo "El script se debe usar con sudo"
  exit 1
fi

echo "Se van a instalar las dependencias..."

if [ -f /etc/os-release ]; then
  . /etc/os-release
  DISTRO=$ID
else
  echo "No se puede detectar la distribución!"
  exit 1
fi

echo "Distribución detectada: $DISTRO"

echo "Instalando dependencias..."
if [ "$DISTRO" == "ubuntu" ] || [ $DISTRO == "debian" ]; then
  echo "Instalando con apt..."
  apt update
  apt install -y nasm make binutils qemu-system-x86 curl
elif [ $DISTRO == "fedora" ]; then
  echo "Instalando con dnf..."
  dnf install -y nasm make binutils qemu-system-x86 curl
else
  echo "No se soporta $DISTRO, instala las dependencias manualmente"
  echo "es necesario rust, usa el script de instalación de su web"
  echo "nasm make binutils qemu-system-x86 curl"
fi

USER_HOME=$(getent passwd $SUDO_USER | cut -d : -f 6)

echo "Instalando Rust para el usuario $SUDO_USER en $USER_HOME"
sudo -u $SUDO_USER bash -c '
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . $HOME/.cargo/env
  rustup override set nightly
  rustup component add rust-src
  echo "Dependencias instaladas. Para ejecutar el proyecto usa \"make run\""
  exec $SHELL
'