


;;= malloc
extern malloc
%macro ocmalloc 2
	mov rcx,%1
	sub rsp,8
	call malloc
	add rsp,8
	mov %2,rax
%endmacro


;;= free
extern free
%macro ocfree 1
	mov rcx,%1
	sub rsp,32
	call free
	add rsp,32
%endmacro