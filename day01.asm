%include "sys.asm"
%include "lib.asm"

%define rPosX r8
%define rPosY r9
%define rDirX r10
%define rDirY r11

global _start
_start:
  sub rsp, 4096
  syscall SYS_READ, FD_STDIN, rsp, 4096
  mov rdi, rax ;; Length.
  xor rcx, rcx ;; Index.

  xor rPosX, rPosX
  xor rPosY, rPosY
  xor rDirX, rDirX
  mov rDirY, -1 ;; North.

  .loopTurn:
    cmp byte [rsp + rcx], 'L'
    jne .right
    .left: ;; L(dx, dy) = (dy, -dx)
      neg rDirX
      jmp .swap
    .right: ;; R(dx, dy) = (-dy, dx)
      neg rDirY
    .swap:
    xchg rDirX, rDirY

    xor rax, rax
    .loopDigit:
      inc rcx
      movzx rdx, byte [rsp + rcx]
      cmp dl, '0'
      jb .breakDigit

      ;; rax = rax * 10 + rdx - '0'
      shl rax, 1
      lea rax, [rax + rax * 4 - '0']
      add rax, rdx
    jmp .loopDigit
    .breakDigit:
    add rcx, 2 ;; Discard comma and space.

    mov rdx, rax
    imul rdx, rDirX
    add rPosX, rdx

    mov rdx, rax
    imul rdx, rDirY
    add rPosY, rdx

  cmp rcx, rdi
  jb .loopTurn

  ;; abs(x) = (x ^ (x >> 63)) - (x >> 63)
  mov rax, rPosX
  sar rax, 63
  xor rPosX, rax
  sub rPosX, rax

  mov rax, rPosY
  sar rax, 63
  xor rPosY, rax
  sub rPosY, rax

  lea rdi, [rPosX + rPosY]
  call hex32
  push rax
  syscall SYS_WRITE, FD_STDOUT, rsp, 8

  xor rax, rax
syscall SYS_EXIT, rax
