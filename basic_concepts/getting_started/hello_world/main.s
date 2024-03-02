	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"main.aaccb3f0a8cfdec6-cgu.0"
	.def	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4d28f88769ff0699E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4d28f88769ff0699E
	.p2align	4, 0x90
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4d28f88769ff0699E:
.seh_proc _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4d28f88769ff0699E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	_ZN4core3ops8function6FnOnce9call_once17h075c253ec62ed82aE
	#APP
	#NO_APP
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start17ha354c19399061396E;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17ha354c19399061396E
	.globl	_ZN3std2rt10lang_start17ha354c19399061396E
	.p2align	4, 0x90
_ZN3std2rt10lang_start17ha354c19399061396E:
.seh_proc _ZN3std2rt10lang_start17ha354c19399061396E
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movb	%r9b, %al
	movq	%r8, %r9
	movq	%rdx, %r8
	movq	%rcx, 48(%rsp)
	leaq	48(%rsp), %rcx
	leaq	__unnamed_1(%rip), %rdx
	movb	%al, 32(%rsp)
	callq	_ZN3std2rt19lang_start_internal17h125867de5e07cdbcE
	movq	%rax, 40(%rsp)
	movq	40(%rsp), %rax
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E
	.p2align	4, 0x90
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h4d28f88769ff0699E
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h805bfd6df14db643E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E
	.p2align	4, 0x90
_ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E:
.seh_proc _ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E
	subq	$136, %rsp
	.seh_stackalloc 136
	.seh_endprologue
	movq	%r8, 40(%rsp)
	movq	%rdx, 48(%rsp)
	movq	%rcx, 56(%rsp)
	movq	%rcx, 64(%rsp)
	cmpq	$1, %r8
	ja	.LBB3_2
	movq	64(%rsp), %rax
	movq	56(%rsp), %rcx
	movq	40(%rsp), %rdx
	movq	48(%rsp), %r8
	movq	$0, 120(%rsp)
	movq	%r8, (%rcx)
	movq	%rdx, 8(%rcx)
	movq	120(%rsp), %r8
	movq	128(%rsp), %rdx
	movq	%r8, 32(%rcx)
	movq	%rdx, 40(%rcx)
	leaq	__unnamed_2(%rip), %rdx
	movq	%rdx, 16(%rcx)
	movq	$0, 24(%rcx)
	addq	$136, %rsp
	retq
.LBB3_2:
	leaq	__unnamed_3(%rip), %rdx
	leaq	72(%rsp), %rcx
	movq	%rcx, 32(%rsp)
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E
	movq	32(%rsp), %rcx
	leaq	__unnamed_4(%rip), %rdx
	callq	_ZN4core9panicking9panic_fmt17hf9ca4b93f563e888E
	ud2
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9698054b1f0a54f3E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9698054b1f0a54f3E
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9698054b1f0a54f3E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9698054b1f0a54f3E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce9call_once17h075c253ec62ed82aE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17h075c253ec62ed82aE
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce9call_once17h075c253ec62ed82aE:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17h075c253ec62ed82aE
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E:
.Lfunc_begin0:
.seh_proc _ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	.seh_handler __CxxFrameHandler3, @unwind, @except
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$64, %rsp
	.seh_stackalloc 64
	leaq	64(%rsp), %rbp
	.seh_setframe %rbp, 64
	.seh_endprologue
	movq	$-2, -8(%rbp)
	movq	%rcx, -16(%rbp)
.Ltmp0:
	leaq	-16(%rbp), %rcx
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E
.Ltmp1:
	movl	%eax, -20(%rbp)
	jmp	.LBB6_2
.LBB6_2:
	movl	-20(%rbp), %eax
	addq	$64, %rsp
	popq	%rbp
	retq
	.seh_handlerdata
	.long	($cppxdata$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E)@IMGREL
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	.seh_endproc
	.def	"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E@4HA";
	.scl	3;
	.type	32;
	.endef
	.p2align	4, 0x90
"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E@4HA":
.seh_proc "?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E@4HA"
.LBB6_1:
	movq	%rdx, 16(%rsp)
	pushq	%rbp
	.seh_pushreg %rbp
	subq	$32, %rsp
	.seh_stackalloc 32
	leaq	64(%rdx), %rbp
	.seh_endprologue
	addq	$32, %rsp
	popq	%rbp
	retq
.Lfunc_end0:
	.seh_handlerdata
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	.seh_endproc
	.section	.xdata,"dr",associative,_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E
	.p2align	2, 0x0
$cppxdata$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E:
	.long	429065506
	.long	1
	.long	($stateUnwindMap$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E)@IMGREL
	.long	0
	.long	0
	.long	3
	.long	($ip2state$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E)@IMGREL
	.long	56
	.long	0
	.long	1
$stateUnwindMap$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E:
	.long	-1
	.long	"?dtor$1@?0?_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E@4HA"@IMGREL
$ip2state$_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E:
	.long	.Lfunc_begin0@IMGREL
	.long	-1
	.long	.Ltmp0@IMGREL+1
	.long	0
	.long	.Ltmp1@IMGREL+1
	.long	-1
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce9call_once17hd41aba6c4937dc53E

	.def	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h4487fcdff88bdf98E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h4487fcdff88bdf98E
	.p2align	4, 0x90
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h4487fcdff88bdf98E:
	retq

	.def	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h805bfd6df14db643E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h805bfd6df14db643E
	.p2align	4, 0x90
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h805bfd6df14db643E:
	xorl	%eax, %eax
	retq

	.def	_ZN4main4main17hd80a274883898d96E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4main4main17hd80a274883898d96E
	.p2align	4, 0x90
_ZN4main4main17hd80a274883898d96E:
.seh_proc _ZN4main4main17hd80a274883898d96E
	subq	$88, %rsp
	.seh_stackalloc 88
	.seh_endprologue
	leaq	40(%rsp), %rcx
	leaq	__unnamed_5(%rip), %rdx
	movl	$1, %r8d
	callq	_ZN4core3fmt9Arguments9new_const17h4b2ecf046b247fb0E
	leaq	40(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17h52999c199d5d532aE
	nop
	addq	$88, %rsp
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main
	.globl	main
	.p2align	4, 0x90
main:
.seh_proc main
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	%rdx, %r8
	movslq	%ecx, %rdx
	leaq	_ZN4main4main17hd80a274883898d96E(%rip), %rcx
	xorl	%r9d, %r9d
	callq	_ZN3std2rt10lang_start17ha354c19399061396E
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3, 0x0
__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h4487fcdff88bdf98E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9698054b1f0a54f3E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17heaac80a69f899197E

	.section	.rdata,"dr",one_only,__unnamed_2
	.p2align	3, 0x0
__unnamed_2:

	.section	.rdata,"dr",one_only,__unnamed_6
__unnamed_6:
	.ascii	"invalid args"

	.section	.rdata,"dr",one_only,__unnamed_3
	.p2align	3, 0x0
__unnamed_3:
	.quad	__unnamed_6
	.asciz	"\f\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,__unnamed_7
__unnamed_7:
	.ascii	"/rustc/79e9716c980570bfd1f666e3b16ac583f0168962\\library\\core\\src\\fmt\\mod.rs"

	.section	.rdata,"dr",one_only,__unnamed_4
	.p2align	3, 0x0
__unnamed_4:
	.quad	__unnamed_7
	.asciz	"K\000\000\000\000\000\000\000?\001\000\000\r\000\000"

	.section	.rdata,"dr",one_only,__unnamed_8
__unnamed_8:
	.ascii	"Hello World\n"

	.section	.rdata,"dr",one_only,__unnamed_5
	.p2align	3, 0x0
__unnamed_5:
	.quad	__unnamed_8
	.asciz	"\f\000\000\000\000\000\000"

