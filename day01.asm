%include "sys.asm"
global _start

section .bss
input: resb 1

section .data
position:
  .x: dd 0
  .y: dd 0
direction:
  .x: dd 0
  .y: dd -1

section .text
_start:
  .loopTurn:
    syscall SYS_READ, FD_STDIN, input, 1
    test rax, rax
    jz .breakTurn

    cmp byte [input], 'R'
    jne .elseLeft
    .thenRight:
      rol qword [direction], 32
      neg dword [direction.x]
      jmp .endLeft
    .elseLeft:
      neg dword [direction.x]
      rol qword [direction], 32
    .endLeft:

    xor r12, r12
    .loopDigit:
      syscall SYS_READ, FD_STDIN, input, 1
      test rax, rax
      jz .breakDigit
      movzx rax, byte [input]

      cmp al, ','
      je .breakDigit

      shl r12, 1
      lea r12, [r12 + r12 * 4 - '0']
      add r12, rax
    jmp .loopDigit
    .breakDigit:

    syscall SYS_READ, FD_STDIN, input, 1

    mov eax, r12d
    imul eax, [direction.x]
    add [position.x], eax
    imul r12d, [direction.y]
    add [position.y], r12d
  jmp .loopTurn
  .breakTurn:

  mov eax, [position.x]
  sar eax, 31
  mov ecx, eax
  xor ecx, [position.x]
  sub ecx, eax

  mov eax, [position.y]
  sar eax, 31
  mov edx, eax
  xor edx, [position.y]
  sub edx, eax

  add ecx, edx
syscall SYS_EXIT, rcx
