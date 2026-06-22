ENTRY(hypervisor_main)

KERNEL_VMA = 0xffffffffff800000;

SECTIONS
{
	. = KERNEL_VMA;
	_kernel_vma = .;

	.text : ALIGN(0x1000)
	{
		_code = .;
		*(.text*)
	}

	.rodata : ALIGN(0x1000)
	{
		_rodata = .;
		*(.rodata*)
	}

	.data : ALIGN(0x1000)
	{
		_data = .;
		*(.data*)
	}

	.eh_frame : ALIGN(0x1000)
	{
		_ehframe = .;
		*(.eh_frame*)
	}

	.bss : ALIGN(0x1000)
	{
		_bss = .;
		*(.bss*)

		/*
		 * You usually need to include generated COMMON symbols
		 * under kernel BSS section or use gcc's -fno-common
		 */
		
		*(COMMON)
	}

	_end = .;

	/DISCARD/ :
	{
		*(.comment)
		*(.note*)
		*(.got*)
	}
}
