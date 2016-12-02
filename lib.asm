section .rodata
hexDigits: db '0123456789ABCDEF'

section .text
hex32:
  push rbx
  mov rbx, hexDigits
  xor rax, rax

%rep 8
  shl rax, 8
  mov al, dil
  and al, 0x0F
  xlatb
  shr rdi, 4
%endrep

  pop rbx
ret
