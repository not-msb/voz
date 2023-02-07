global _start
section .text
_start:
push rbp
mov rbp, rsp
push 65
push 1
push 1
l3:
mov rax, [rbp-16]
add rax, [rbp-24]
mov [rbp-16], rax
mov rax, [rbp-16]
cmp rax, [rbp-8]
je l6
mov rax, [rbp-8]
cmp rax, [rbp-8]
jmp l3
l6:
mov rax, 1
mov rdi, 1
sub rbp, 16
mov rsi, rbp
add rbp, 16
mov rdx, 1
syscall
jmp exit
exit:
mov rsp, rbp
pop rbp
mov rax, 60
mov rdi, 0
syscall
