ENTRY(hypervisor_main)

PHDRS
{
        text            PT_LOAD    FLAGS((1 << 0) | (1 << 2));           /* Execute + Read */
        rodata          PT_LOAD    FLAGS((1 << 2));                      /* Read only */
        data            PT_LOAD    FLAGS((1 << 1) | (1 << 2));           /* Write + Read */
}

SECTIONS
{
	_kernel_vma = .;

	.text : ALIGN(0x1000)
	{
		_code = .;
		*(.text*)
	} :text

	.rodata : ALIGN(0x1000)
	{
		_rodata = .;
		*(.rodata*)
	} :rodata

	.data : ALIGN(0x1000)
	{
		_data = .;
		*(.data*)
	} :data

	.bss : ALIGN(0x1000)
	{
		_bss = .;
		*(.bss*)

		/*
		 * You usually need to include generated COMMON symbols
		 * under kernel BSS section or use gcc's -fno-common
		 */
		
		*(COMMON)
	} :data

	_end = .;

	/DISCARD/ :
	{
		*(.comment)
		*(.note*)
		*(.got*)
	}
}
