%macro syscall 0
  syscall
%endmacro

%macro syscall 1
  mov rax, %1
  syscall
%endmacro

%macro syscall 2
  mov rdi, %2
  syscall %1
%endmacro

%macro syscall 3
  mov rsi, %3
  syscall %1, %2
%endmacro

%macro syscall 4
  mov rdx, %4
  syscall %1, %2, %3
%endmacro

%macro syscall 5
  mov r10, %5
  syscall %1, %2, %3, %4
%endmacro

%macro syscall 6
  mov r8, %6
  syscall %1, %2, %3, %4, %5
%endmacro

%macro syscall 7
  mov r9, %7
  syscall %1, %2, %3, %4, %5, %6
%endmacro

SYS_READ   equ 0
SYS_WRITE  equ 1
SYS_EXIT   equ 60

FD_STDIN  equ 0
FD_STDOUT equ 1
FD_STDERR equ 2
